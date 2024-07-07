use super::Proposal;

pub struct ZKPProof {
    pub commitment: i32,
    pub response: i32,
}

pub fn generate_proof(secret: i32, min: i32, max: i32) -> ZKPProof {
    let nonce: i32 = rand::random::<i32>() % 100; // random nonce for the proof
    let commitment = secret + nonce; // simple commitment
    let challenge = rand::random::<i32>() % (max - min) + min; // random challenge within the range
    let response = nonce + challenge; // response to the challenge

    ZKPProof {
        commitment,
        response,
    }
}

pub fn verify_proof(proof: &ZKPProof, min: i32, max: i32) -> bool {
    let challenge = proof.response - proof.commitment;
    challenge >= min && challenge <= max
}

pub fn verify_proposal_approval(proposal: &Proposal) -> bool {
    // Example: Verify that proposal title length is between 5 and 15
    let proof = generate_proof(proposal.title.len() as i32, 5, 15);
    verify_proof(&proof, 5, 15)
}
