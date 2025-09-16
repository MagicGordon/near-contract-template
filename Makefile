build: lint {{contract-name}} mock_ft

lint:
	@cargo fmt --all
	@cargo clippy --fix --allow-dirty --allow-staged --features=test


{{contract-name}}: contracts/{{contract-name}}
	$(call local_build_wasm,{{contract-name}},{{contract-name}},test)

mock_ft: contracts/mock_ft
	$(call local_build_wasm,mock_ft,mock_ft)

count:
	@tokei ./contracts/{{contract-name}}/src/ --files --exclude unit

release:
	$(call build_release_wasm,{{contract-name}},{{contract-name}})

clean:
	cargo clean
	rm -rf res/

unittest: build
ifdef TC
	cargo nextest run --package proxy_account $(TC)
else
	cargo nextest run --package proxy_account --lib
endif

test: build
ifdef TF
	cargo nextest run --package proxy_account --test $(TF)
else
	cargo nextest run --package proxy_account --tests
endif

define local_build_wasm
	$(eval PACKAGE_NAME := $(1))
	$(eval WASM_NAME := $(2))
	$(eval FEATURES := $(3))

	@mkdir -p res
	@rustup target add wasm32-unknown-unknown
	@if [ -n "$(FEATURES)" ]; then \
		cargo near build non-reproducible-wasm --manifest-path ./contracts/${PACKAGE_NAME}/Cargo.toml --features=$(FEATURES) --locked --no-abi; \
	else \
		cargo near build non-reproducible-wasm --manifest-path ./contracts/${PACKAGE_NAME}/Cargo.toml --locked --no-abi; \
	fi
	@cp target/near/${WASM_NAME}/$(WASM_NAME).wasm ./res/$(WASM_NAME).wasm
endef

define build_release_wasm
	$(eval PACKAGE_NAME := $(1))
	$(eval WASM_NAME := $(2))

	@mkdir -p res
	@rustup target add wasm32-unknown-unknown
	@cargo near build reproducible-wasm --manifest-path ./contracts/${PACKAGE_NAME}/Cargo.toml
	@cp target/near/${WASM_NAME}/$(WASM_NAME).wasm ./res/$(WASM_NAME)_release.wasm
endef