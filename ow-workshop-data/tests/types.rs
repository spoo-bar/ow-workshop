//! Tests for the type rules and value signatures.

use ow_workshop_data::signature::{DefaultValue, is_subtype};
use ow_workshop_data::{Ty, ValueId, VectorKind};

#[test]
fn numbers_are_subtypes_of_float() {
    assert!(is_subtype(&Ty::UnsignedInt, &Ty::Float));
    assert!(is_subtype(&Ty::UnsignedInt, &Ty::Object));
    assert!(!is_subtype(&Ty::Float, &Ty::UnsignedInt));
}

#[test]
fn unions_need_every_member_to_fit() {
    let mixed = Ty::Union(&[Ty::Float, Ty::Vector]);
    assert!(!is_subtype(&mixed, &Ty::Float));
    assert!(is_subtype(&mixed, &Ty::Object));
    assert!(is_subtype(&Ty::Float, &mixed));
}

const PLAYERS: Ty = Ty::ArrayOf(&Ty::Player);
const PLAYER_OR_PLAYERS: Ty = Ty::Union(&[Ty::Player, PLAYERS]);

#[test]
fn arrays_only_fit_arrays() {
    assert!(!is_subtype(&PLAYERS, &Ty::Player));
    assert!(is_subtype(&PLAYERS, &Ty::Array));
    assert!(is_subtype(&PLAYERS, &PLAYER_OR_PLAYERS));
    assert!(is_subtype(&PLAYERS, &Ty::ArrayOf(&Ty::Object)));
    assert!(!is_subtype(&Ty::ArrayOf(&Ty::Object), &PLAYERS));
}

#[test]
fn typed_vectors_fit_vector_when_components_are_numbers() {
    let numbers = Ty::Vec3(VectorKind::Vector, &[Ty::Float, Ty::UnsignedInt, Ty::Int]);
    assert!(is_subtype(&numbers, &Ty::Vector));
    assert!(is_subtype(&numbers, &Ty::Position));
    let mixed = Ty::Vec3(VectorKind::Vector, &[Ty::Float, Ty::Player, Ty::Int]);
    assert!(!is_subtype(&mixed, &Ty::Vector));
}

#[test]
fn literals_are_subtypes_of_their_base() {
    assert!(is_subtype(&Ty::HeroLiteral, &Ty::Hero));
    assert!(is_subtype(&Ty::CustomStringLiteral, &Ty::String));
}

#[test]
fn value_signatures_are_complete() {
    assert_eq!(ValueId::ALL.len(), 267);
    let all_players = ValueId::GetPlayers.signature();
    assert_eq!(all_players.args.len(), 1);
    assert_eq!(
        all_players.args[0].default,
        Some(DefaultValue::Const("Team", "ALL"))
    );
}
