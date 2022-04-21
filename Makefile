.PHONY: entity

entity:
	@cargo run --manifest-path ../sea-orm/sea-orm-cli/Cargo.toml -- generate entity -o entity/src/defs --expanded-format