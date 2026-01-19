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

// Test generic enum with type parameter (similar to HydraUserIntentDatum<T>)
#[derive(Clone, Debug, ImplConstr)]
pub struct UserAccount(pub Constr0<Box<(ByteString, ByteString)>>);

#[derive(Clone, Debug, ImplConstr)]
pub struct TransferIntent(pub Constr0<Box<(Int, Int)>>);

#[derive(Debug, Clone, ConstrEnum)]
pub enum GenericDatum<T: PlutusDataJson = PlutusData> {
    TradeIntent(Box<(UserAccount, T)>),
    MasterIntent(Box<(UserAccount, T)>),
}

#[test]
fn test_generic_enum_with_type_parameter() {
    // Test with concrete type (TransferIntent)
    let user = UserAccount::from("user_vk", "app_owner_vk");
    let intent = TransferIntent::from(100, 200);
    let datum: GenericDatum<TransferIntent> =
        GenericDatum::MasterIntent(Box::new((user.clone(), intent)));

    let json = datum.to_json_string();
    assert!(json.contains("\"constructor\":1")); // MasterIntent is constructor 1
    assert!(json.contains("\"fields\""));

    // Test with default type (PlutusData)
    let datum_default: GenericDatum =
        GenericDatum::TradeIntent(Box::new((user, PlutusData::Integer(Int::new(42)))));

    let json_default = datum_default.to_json_string();
    assert!(json_default.contains("\"constructor\":0")); // TradeIntent is constructor 0
    assert!(json_default.contains("\"int\":42")); // Int value is serialized as number
}

// ============================================================================
// Round-trip from_json tests
// ============================================================================

#[test]
fn test_branch_from_json_roundtrip() {
    let original = Branch::from(42, "abcdef123456");
    let json_str = original.to_json_string();
    let parsed = Branch::from_json_string(&json_str).unwrap();
    assert_eq!(original.to_json_string(), parsed.to_json_string());
}

#[test]
fn test_neighbor_from_json_roundtrip() {
    let original = Neighbor::from(99, "prefix_hash", "suffix_hash");
    let json_str = original.to_json_string();
    let parsed = Neighbor::from_json_string(&json_str).unwrap();
    assert_eq!(original.to_json_string(), parsed.to_json_string());
}

#[test]
fn test_fork_from_json_roundtrip() {
    let neighbor = Neighbor::from(5, "left", "right");
    let original = Fork::from(10, neighbor);
    let json_str = original.to_json_string();
    let parsed = Fork::from_json_string(&json_str).unwrap();
    assert_eq!(original.to_json_string(), parsed.to_json_string());
}

#[test]
fn test_leaf_from_json_roundtrip() {
    let original = Leaf::from(7, "key_hash", "value_hash");
    let json_str = original.to_json_string();
    let parsed = Leaf::from_json_string(&json_str).unwrap();
    assert_eq!(original.to_json_string(), parsed.to_json_string());
}

#[test]
fn test_proof_step_branch_from_json_roundtrip() {
    let original = ProofStep::Branch(Branch::from(1, "hash"));
    let json_str = original.to_json_string();
    let parsed = ProofStep::from_json_string(&json_str).unwrap();
    assert_eq!(original.to_json_string(), parsed.to_json_string());
}

#[test]
fn test_proof_step_fork_from_json_roundtrip() {
    let neighbor = Neighbor::from(2, "a", "b");
    let original = ProofStep::Fork(Fork::from(3, neighbor));
    let json_str = original.to_json_string();
    let parsed = ProofStep::from_json_string(&json_str).unwrap();
    assert_eq!(original.to_json_string(), parsed.to_json_string());
}

#[test]
fn test_proof_step_leaf_from_json_roundtrip() {
    let original = ProofStep::Leaf(Leaf::from(4, "key", "val"));
    let json_str = original.to_json_string();
    let parsed = ProofStep::from_json_string(&json_str).unwrap();
    assert_eq!(original.to_json_string(), parsed.to_json_string());
}

#[test]
fn test_mpf_insert_from_json_roundtrip() {
    let original = MPFInsert::from(vec![
        ProofStep::Branch(Branch::from(1, "abc")),
        ProofStep::Leaf(Leaf::from(2, "def", "ghi")),
    ]);
    let json_str = original.to_json_string();
    let parsed = MPFInsert::from_json_string(&json_str).unwrap();
    assert_eq!(original.to_json_string(), parsed.to_json_string());
}

#[test]
fn test_mpf_update_from_json_roundtrip() {
    let proof = List::new(&[ProofStep::Branch(Branch::from(1, "hash"))]);
    let original = MPFUpdate::from("old", "new", proof);
    let json_str = original.to_json_string();
    let parsed = MPFUpdate::from_json_string(&json_str).unwrap();
    assert_eq!(original.to_json_string(), parsed.to_json_string());
}

#[test]
fn test_mpf_delete_from_json_roundtrip() {
    let proof = List::new(&[ProofStep::Leaf(Leaf::from(5, "k", "v"))]);
    let original = MPFDelete::from(proof);
    let json_str = original.to_json_string();
    let parsed = MPFDelete::from_json_string(&json_str).unwrap();
    assert_eq!(original.to_json_string(), parsed.to_json_string());
}

#[test]
fn test_mpf_proof_insert_from_json_roundtrip() {
    let original = MPFProof::MPFInsert(MPFInsert::from(vec![ProofStep::Branch(Branch::from(
        1, "x",
    ))]));
    let json_str = original.to_json_string();
    let parsed = MPFProof::from_json_string(&json_str).unwrap();
    assert_eq!(original.to_json_string(), parsed.to_json_string());
}

#[test]
fn test_mpf_proof_update_from_json_roundtrip() {
    let original = MPFProof::MPFUpdate(MPFUpdate::from("a", "b", List::new(&[])));
    let json_str = original.to_json_string();
    let parsed = MPFProof::from_json_string(&json_str).unwrap();
    assert_eq!(original.to_json_string(), parsed.to_json_string());
}

#[test]
fn test_mpf_proof_delete_from_json_roundtrip() {
    let original = MPFProof::MPFDelete(MPFDelete::from(List::new(&[])));
    let json_str = original.to_json_string();
    let parsed = MPFProof::from_json_string(&json_str).unwrap();
    assert_eq!(original.to_json_string(), parsed.to_json_string());
}

#[test]
fn test_process_app_deposit_from_json_roundtrip() {
    let proofs = vec![
        MPFProof::MPFInsert(MPFInsert::from(vec![
            ProofStep::Branch(Branch::from(1, "abcdef")),
            ProofStep::Leaf(Leaf::from(2, "123456", "789abc")),
            ProofStep::Fork(Fork::from(3, Neighbor::from(4, "ghijkl", ""))),
        ])),
        MPFProof::MPFUpdate(MPFUpdate::from("old_value", "new_value", List::new(&[]))),
        MPFProof::MPFDelete(MPFDelete::from(List::new(&[]))),
    ];
    let original = ProcessAppDeposit::from(proofs);
    let json_str = original.to_json_string();
    let parsed = ProcessAppDeposit::from_json_string(&json_str).unwrap();
    assert_eq!(original.to_json_string(), parsed.to_json_string());
}

#[test]
fn test_tree_branch_from_json_roundtrip() {
    let original = TreeBranch::from("prefix", PlutusData::Integer(Int::new(100)));
    let json_str = original.to_json_string();
    let parsed = TreeBranch::from_json_string(&json_str).unwrap();
    assert_eq!(original.to_json_string(), parsed.to_json_string());
}

#[test]
fn test_tree_leaf_from_json_roundtrip() {
    let original = TreeLeaf::from("key", "value", "extra");
    let json_str = original.to_json_string();
    let parsed = TreeLeaf::from_json_string(&json_str).unwrap();
    assert_eq!(original.to_json_string(), parsed.to_json_string());
}

#[test]
fn test_tree_enum_branch_from_json_roundtrip() {
    let original = Tree::TreeBranch(TreeBranch::from("node", PlutusData::Integer(Int::new(50))));
    let json_str = original.to_json_string();
    let parsed = Tree::from_json_string(&json_str).unwrap();
    assert_eq!(original.to_json_string(), parsed.to_json_string());
}

#[test]
fn test_tree_enum_leaf_from_json_roundtrip() {
    let original = Tree::TreeLeaf(TreeLeaf::from("a", "b", "c"));
    let json_str = original.to_json_string();
    let parsed = Tree::from_json_string(&json_str).unwrap();
    assert_eq!(original.to_json_string(), parsed.to_json_string());
}

#[test]
fn test_full_tree_from_json_roundtrip() {
    let trees = vec![
        Tree::TreeBranch(TreeBranch::from("n1", PlutusData::Integer(Int::new(1)))),
        Tree::TreeLeaf(TreeLeaf::from("k1", "v1", "e1")),
        Tree::TreeBranch(TreeBranch::from(
            "n2",
            PlutusData::ByteString(ByteString::new("deadbeef")),
        )),
    ];
    let original = FullTree::from(trees);
    let json_str = original.to_json_string();
    let parsed = FullTree::from_json_string(&json_str).unwrap();
    assert_eq!(original.to_json_string(), parsed.to_json_string());
}

#[test]
fn test_proofs_from_json_roundtrip() {
    let mpf_proofs = vec![MPFProof::MPFInsert(MPFInsert::from(vec![
        ProofStep::Branch(Branch::from(1, "h")),
    ]))];
    let original = Proofs::from(mpf_proofs);
    let json_str = original.to_json_string();
    let parsed = Proofs::from_json_string(&json_str).unwrap();
    assert_eq!(original.to_json_string(), parsed.to_json_string());
}

#[test]
fn test_tree_or_proofs_full_tree_from_json_roundtrip() {
    let trees = vec![Tree::TreeLeaf(TreeLeaf::from("x", "y", "z"))];
    let original = TreeOrProofs::FullTree(FullTree::from(trees));
    let json_str = original.to_json_string();
    let parsed = TreeOrProofs::from_json_string(&json_str).unwrap();
    assert_eq!(original.to_json_string(), parsed.to_json_string());
}

#[test]
fn test_tree_or_proofs_proofs_from_json_roundtrip() {
    let mpf_proofs = vec![MPFProof::MPFDelete(MPFDelete::from(List::new(&[])))];
    let original = TreeOrProofs::Proofs(Proofs::from(mpf_proofs));
    let json_str = original.to_json_string();
    let parsed = TreeOrProofs::from_json_string(&json_str).unwrap();
    assert_eq!(original.to_json_string(), parsed.to_json_string());
}

#[test]
fn test_user_account_from_json_roundtrip() {
    let original = UserAccount::from("user_verification_key", "app_owner_key");
    let json_str = original.to_json_string();
    let parsed = UserAccount::from_json_string(&json_str).unwrap();
    assert_eq!(original.to_json_string(), parsed.to_json_string());
}

#[test]
fn test_transfer_intent_from_json_roundtrip() {
    let original = TransferIntent::from(1000, 2000);
    let json_str = original.to_json_string();
    let parsed = TransferIntent::from_json_string(&json_str).unwrap();
    assert_eq!(original.to_json_string(), parsed.to_json_string());
}

#[test]
fn test_generic_datum_trade_intent_from_json_roundtrip() {
    let user = UserAccount::from("user1", "owner1");
    let intent = TransferIntent::from(500, 600);
    let original: GenericDatum<TransferIntent> =
        GenericDatum::TradeIntent(Box::new((user, intent)));
    let json_str = original.to_json_string();
    let parsed = GenericDatum::<TransferIntent>::from_json_string(&json_str).unwrap();
    assert_eq!(original.to_json_string(), parsed.to_json_string());
}

#[test]
fn test_generic_datum_master_intent_from_json_roundtrip() {
    let user = UserAccount::from("user2", "owner2");
    let intent = TransferIntent::from(700, 800);
    let original: GenericDatum<TransferIntent> =
        GenericDatum::MasterIntent(Box::new((user, intent)));
    let json_str = original.to_json_string();
    let parsed = GenericDatum::<TransferIntent>::from_json_string(&json_str).unwrap();
    assert_eq!(original.to_json_string(), parsed.to_json_string());
}

#[test]
fn test_generic_datum_with_plutus_data_from_json_roundtrip() {
    let user = UserAccount::from("user3", "owner3");
    let original: GenericDatum<PlutusData> =
        GenericDatum::TradeIntent(Box::new((user, PlutusData::Integer(Int::new(999)))));
    let json_str = original.to_json_string();
    let parsed = GenericDatum::<PlutusData>::from_json_string(&json_str).unwrap();
    assert_eq!(original.to_json_string(), parsed.to_json_string());
}
