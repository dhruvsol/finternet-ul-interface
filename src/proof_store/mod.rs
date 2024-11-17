// proof store spec

use thiserror::Error;

#[derive(Error, Debug)]
pub enum FPoofStoreError {}

#[derive(Clone, Debug)]
pub struct ULProof;

#[derive(Clone, Debug)]
pub struct ULProofID;

/// A trait defining operations for storing and verifying Unified Ledger proofs
/// 
/// The FPoofStore provides a consistent interface for managing cryptographic proofs
/// across different storage implementations. It ensures that proofs can be reliably
/// stored, retrieved, and verified regardless of the underlying storage mechanism.
pub trait FPoofStore {

    /// Stores a proof with its associated ID
    /// 
    /// # Arguments
    /// * `key` - The unique identifier for the proof
    /// * `proof` - The proof to store
    /// 
    /// # Returns
    /// * `Ok(())` if the proof was successfully stored
    /// * `Err(FPoofStoreError)` if the storage operation failed
    /// 
    /// # Example
    /// ```no_run
    /// # use your_crate::{FPoofStore, ULProof, ULProofID, FPoofStoreError};
    /// # struct MyStore;
    /// # impl FPoofStore for MyStore {
    /// #     fn set(&mut self, key: ULProofID, proof: ULProof) -> Result<(), FPoofStoreError> { Ok(()) }
    /// #     fn get(&mut self, key: ULProofID, proof: ULProof) -> Result<Option<ULProof>, FPoofStoreError> { Ok(None) }
    /// #     fn verify(&mut self, proof: ULProof) -> Result<bool, FPoofStoreError> { Ok(false) }
    /// # }
    /// # let mut store = MyStore;
    /// # let proof_id = ULProofID;
    /// # let proof = ULProof;
    /// store.set(proof_id, proof)?;
    /// ```
    fn set(&mut self, key: ULProofID, proof: ULProof) -> Result<(), FPoofStoreError>;


    
    /// Retrieves a proof by its ID
    /// 
    /// # Arguments
    /// * `key` - The unique identifier of the proof to retrieve
    /// * `proof` - The proof instance to validate against (for verification purposes)
    /// 
    /// # Returns
    /// * `Ok(Some(ULProof))` if the proof was found
    /// * `Ok(None)` if no proof exists for the given ID
    /// * `Err(FPoofStoreError)` if the retrieval operation failed
    /// 
    /// # Example
    /// ```no_run
    /// # use your_crate::{FPoofStore, ULProof, ULProofID, FPoofStoreError};
    /// # struct MyStore;
    /// # impl FPoofStore for MyStore {
    /// #     fn set(&mut self, key: ULProofID, proof: ULProof) -> Result<(), FPoofStoreError> { Ok(()) }
    /// #     fn get(&mut self, key: ULProofID, proof: ULProof) -> Result<Option<ULProof>, FPoofStoreError> { Ok(None) }
    /// #     fn verify(&mut self, proof: ULProof) -> Result<bool, FPoofStoreError> { Ok(false) }
    /// # }
    /// # let mut store = MyStore;
    /// # let proof_id = ULProofID;
    /// # let proof = ULProof;
    /// if let Some(stored_proof) = store.get(proof_id, proof)? {
    ///     println!("Found proof: {:?}", stored_proof);
    /// }
    /// ```
    fn get(&mut self, key:ULProofID, proof:ULProof) -> Result<Option<ULProof>,FPoofStoreError>;

    /// Verifies the authenticity and integrity of a proof
    /// 
    /// # Arguments
    /// * `proof` - The proof to verify
    /// 
    /// # Returns
    /// * `Ok(true)` if the proof is valid
    /// * `Ok(false)` if the proof is invalid
    /// * `Err(FPoofStoreError)` if the verification operation failed
    /// 
    /// # Example
    /// ```no_run
    /// # use your_crate::{FPoofStore, ULProof, ULProofID, FPoofStoreError};
    /// # struct MyStore;
    /// # impl FPoofStore for MyStore {
    /// #     fn set(&mut self, key: ULProofID, proof: ULProof) -> Result<(), FPoofStoreError> { Ok(()) }
    /// #     fn get(&mut self, key: ULProofID, proof: ULProof) -> Result<Option<ULProof>, FPoofStoreError> { Ok(None) }
    /// #     fn verify(&mut self, proof: ULProof) -> Result<bool, FPoofStoreError> { Ok(false) }
    /// # }
    /// # let mut store = MyStore;
    /// # let proof = ULProof;
    /// if store.verify(proof)? {
    ///     println!("Proof is valid");
    /// } else {
    ///     println!("Proof is invalid");
    /// }
    /// ```
    fn verify(&mut self, proof:ULProof) -> Result<bool,FPoofStoreError>;
}
