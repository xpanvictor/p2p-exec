use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{RwLock};

use tokio::sync::broadcast;

pub struct EventBus {
    subscribers: RwLock<HashMap<TypeId, Box<dyn Any + Send + Sync>>>
}

impl Default for EventBus {
    fn default() -> Self {
        EventBus { subscribers: RwLock::new(HashMap::new()) }
    }
}

impl EventBus {
    pub fn subscribe<T: Clone + Send + Sync + 'static>(&self) -> broadcast::Receiver<T> {
        let type_id = TypeId::of::<T>();
        let mut subs = self.subscribers
            .write()
            .expect("subscribers list error");
        let entry = subs.entry(type_id)
            .or_insert_with(|| {
            let (tx, _) = broadcast::channel::<T>(1024);
            Box::new(tx)
        });
        entry.downcast_ref::<broadcast::Sender<T>>()
            .expect("type integrity guaranteed by TypeId")
            .subscribe()
    }

    pub fn publish<T: Clone + Send + Sync + 'static>(&self, event: T) {
        let subs = self.subscribers.write().expect("subscribers list");
        let type_id = TypeId::of::<T>();
        if let Some(tx_any) = subs.get(&type_id) {
            let tx = tx_any
                .downcast_ref::<broadcast::Sender<T>>()
                .expect("type integrity TypeID");
            let _ = tx.send(event);
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tokio::task;
    use std::sync::Arc;


    #[tokio::test]
    async fn test_concurrent_subscribe() {
        let bus = Arc::new(EventBus::default());
        let mut handles = vec![];

        for _ in 0..10 {
            let bus_clone = bus.clone();
            handles.push(task::spawn(async move {
                let _rx: broadcast::Receiver<String> = bus_clone.subscribe();
            }));
        }

        for handle in handles {
            handle.await.unwrap();
        }

        // Verify that only one sender was created
        let subs = bus.subscribers.read().unwrap();
        assert_eq!(subs.len(), 1);
        assert!(subs.contains_key(&TypeId::of::<String>()));
    }

    #[tokio::test]
    async fn test_concurrent_publish() {
        let bus = Arc::new(EventBus::default());
        let mut rx = bus.subscribe::<String>();

        let bus_clone = bus.clone();
        let publish_handle = task::spawn(async move {
            for i in 0..10 {
                bus_clone.publish(format!("message {}", i));
            }
        });

        publish_handle.await.unwrap();

        // Collect received messages
        let mut received = vec![];
        while let Ok(msg) = rx.try_recv() {
            received.push(msg);
        }
        assert_eq!(received.len(), 10);
    }

    #[tokio::test]
    async fn test_subscribe_and_publish_race() {
        let bus = Arc::new(EventBus::default());
        let mut handles = vec![];

        // Spawn tasks that subscribe and publish concurrently
        for i in 0..5 {
            let bus_clone = bus.clone();
            handles.push(task::spawn(async move {
                let mut rx = bus_clone.subscribe::<String>();
                bus_clone.publish(format!("msg {}", i));
                // Try to receive the message
                if let Ok(msg) = rx.recv().await {
                    assert!(msg.starts_with("msg "));
                }
            }));
        }

        for handle in handles {
            handle.await.unwrap();
        }
    }

    #[tokio::test]
    async fn test_multiple_types_no_race() {
        let bus = Arc::new(EventBus::default());

        let bus1 = bus.clone();
        let bus2 = bus.clone();

        let handle1 = task::spawn(async move {
            let mut rx = bus1.subscribe::<i32>();
            bus1.publish(42i32);
            assert_eq!(rx.recv().await.unwrap(), 42);
        });

        let handle2 = task::spawn(async move {
            let mut rx = bus2.subscribe::<String>();
            bus2.publish("hello".to_string());
            assert_eq!(rx.recv().await.unwrap(), "hello");
        });

        handle1.await.unwrap();
        handle2.await.unwrap();
    }
}
