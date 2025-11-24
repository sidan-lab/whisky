use whisky::data::{
    ByteString, Constr0, Constr1, Constr2, Int, List, Map, PlutusData, PlutusDataJson, Tuple,
};
use whisky::{ConstrEnum, ImplConstr};

#[derive(Clone, Debug, ImplConstr)]
pub struct MPFInsert(pub Constr0<Box<List<ProofStep>>>);

#[derive(Clone, Debug, ImplConstr)]
pub struct MPFUpdate(pub Constr1<Box<(ByteString, ByteString, Proof)>>);

#[derive(Clone, Debug, ImplConstr)]
pub struct MPFDelete(pub Constr2<Box<Proof>>);

pub type Proof = List<ProofStep>;

#[derive(Debug, Clone, ConstrEnum)]
pub enum ProofStep {
    Branch(Branch),
    Fork(Fork),
    Leaf(Leaf),
}

#[derive(Debug, Clone, ConstrEnum)]
pub enum MPFProof {
    MPFInsert(MPFInsert),
    MPFUpdate(MPFUpdate),
    MPFDelete(MPFDelete),
}

#[derive(Clone, Debug, ImplConstr)]
pub struct Branch(pub Constr0<Box<(Int, ByteString)>>);

#[derive(Clone, Debug, ImplConstr)]
pub struct Fork(pub Constr1<Box<(Int, Neighbor)>>);

#[derive(Clone, Debug, ImplConstr)]
pub struct Neighbor(pub Constr0<Box<(Int, ByteString, ByteString)>>);

#[derive(Clone, Debug, ImplConstr)]
pub struct Leaf(pub Constr2<Box<(Int, ByteString, ByteString)>>);

#[derive(Debug, Clone, ImplConstr)]
pub struct ProcessAppDeposit(pub Constr0<Box<List<MPFProof>>>);

#[test]
fn test_complex_generic_type() {
    let proofs = vec![
        MPFProof::MPFInsert(MPFInsert::from(vec![
            ProofStep::Branch(Branch::from(1, "abcdef")),
            ProofStep::Leaf(Leaf::from(2, "123456", "789abc")),
            ProofStep::Fork(Fork::from(3, Neighbor::from(4, "ghijkl", ""))),
        ])),
        MPFProof::MPFUpdate(MPFUpdate::from("old_value", "new_value", List::new(&[]))),
        MPFProof::MPFDelete(MPFDelete::from(List::new(&[]))),
    ];
    let deposit = ProcessAppDeposit::from(proofs);

    let json = deposit.to_json_string();

    // Verify it produces valid JSON with constructor 0 and a list field
    assert!(json.contains("\"constructor\":0"));
    assert!(json.contains("\"fields\""));
    assert!(json.contains("\"list\""));
}

pub type TokenMap = Map<ByteString, Tuple>;

#[derive(Clone, Debug, ImplConstr)]
pub struct TreeOrProofsWithTokenMap(pub Constr0<Box<(TreeOrProofs, TokenMap)>>);

#[derive(Debug, Clone, ConstrEnum)]
pub enum TreeOrProofs {
    FullTree(FullTree),
    Proofs(Proofs),
}

#[derive(Debug, Clone, ImplConstr)]
pub struct FullTree(pub Constr0<Box<List<Tree>>>);

#[derive(Clone, Debug, ImplConstr)]
pub struct Proofs(pub Constr1<Box<List<MPFProof>>>);

#[derive(Debug, Clone, ConstrEnum)]
pub enum Tree {
    TreeBranch(TreeBranch),
    TreeLeaf(TreeLeaf),
}

#[derive(Clone, Debug, ImplConstr)]
pub struct TreeBranch(pub Constr0<Box<(ByteString, PlutusData)>>);

#[derive(Clone, Debug, ImplConstr)]
pub struct TreeLeaf(pub Constr1<Box<(ByteString, ByteString, ByteString)>>);
