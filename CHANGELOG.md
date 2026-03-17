# Changelog

All notable changes to the Whisky project under the Catalyst proposal will be documented in this file.

## [v1.0.28-beta.1] - 2026-03-15 — Milestone 5: Documentation and Closeout Report

### Added

- SECURITY.md — security policy and responsible disclosure process
- MAINTAINERS.md — standalone maintainer list and responsibilities
- CHANGELOG.md — project changelog

### Changed

- Renamed CONTRIBUTING to CONTRIBUTING.md

## [v1.0.27-beta.1] - 2026-02-26 — Milestone 4: Full Pallas Migration

### Changed

- Whisky main package fully migrated to Pallas over CSL

### Added

- Pallas evaluation utility
- Address utilities (address utils, script to address)
- Time convertor utility
- Check required signatures
- Extract UTXOs utility
- Data from_cbor support
- Plutus data from JSON
- serialize_address_obj utility
- MIGRATION.md — CSL to Pallas migration guide

### Fixed

- Redeemer duplicate add
- JSON round trip
- Send + Sync for TxBuildable trait

## [v1.0.24] - 2026-01-22 — Milestone 3: Transaction Parser

### Added

- Transaction parser: parse CBOR hex into editable TxBuilderBody (whisky-pallas)
- Parse all tx components: inputs, outputs, mints, withdrawals, certs, metadata, validity range, votes, collaterals, reference inputs, required signers
- Transaction edit support and tests
- CI: add whisky-pallas to publish workflow

## [v1.0.18-beta.1] - 2026-01-05 — Milestone 2: Transaction Builder with Pallas

### Added

- Pallas as alternative serialization library alongside CSL
- Full tx builder: inputs, outputs, mints, withdrawals, certs, collaterals, voting, required signers, witness set
- Transaction balancing, fee calculation, script data hash
- Feature flag support
- Cipher decrypt with salt
- Integration tests for Pallas
- End-to-end WhiskyPallas tx build documentation in README

### Breaking

- TxBuilderParam requires `serializer: Box<dyn TxBuildable>` field

## [v1.0.17] - 2025-12-17 — Milestone 1: Preparation and Organization Setup

### Added

- ConstrEnum wrapper on enum
- Maintainer list and CODE_OF_CONDUCT.md
- Discord server invite link
- Pallas transaction type prototyping
