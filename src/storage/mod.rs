use std::fmt::Debug;
use crate::key::ul_key::UlID;
use thiserror::{ Error};


#[derive(Error,Debug)]
pub enum FStorageError {}

pub trait FStorage<K,V>
where 
V: Clone + Debug,
{
 
    /// Stores a value associated with a UlId
    /// 
    /// # Arguments
    /// * `key` - The UlId under which to store the value
    /// * `value` - The value to store
    /// 
    /// # Returns
    /// * `Ok(())` if the operation was successful
    /// * `Err(FStorageError)` if the operation failed
    fn set(&mut self, key: UlID, value: V) -> Result<(), FStorageError>;


    /// Retrieves a value associated with a key
    /// 
    /// # Arguments
    /// * `key` - The key for which to retrieve the value
    /// 
    /// # Returns
    /// * `Ok(Some(V))` if the value was found
    /// * `Ok(None)` if the key doesn't exist
    /// * `Err(FStorageError)` if the operation failed
    fn get(&self, key: K) -> Result<Option<V>, FStorageError>;

    
    
    /// Checks if a key exists in storage
    /// 
    /// # Arguments
    /// * `key` - The key to check
    /// 
    /// # Returns
    /// * `Ok(bool)` indicating whether the key exists
    /// * `Err(FStorageError)` if the operation failed
    fn exists(&self, key: K) -> Result<bool, FStorageError> {
        Ok(self.get(key)?.is_some())
    }


    /// Removes a value associated with a key
    /// 
    /// # Arguments
    /// * `key` - The key to remove
    /// 
    /// # Returns
    /// * `Ok(())` if the operation was successful
    /// * `Err(FStorageError)` if the operation failed
    fn remove(&mut self, key: K) -> Result<(), FStorageError>;
}
