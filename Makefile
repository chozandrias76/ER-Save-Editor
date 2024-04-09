.PHONY: format
format:
	@for file in $$(git diff --name-only HEAD); do \
		if [ -f "$$file" ]; then \
			echo "Formatting $$file" && \
			cargo +nightly fmt -- --unstable-features --skip-children src/main.rs $$file; \
		fi \
	done
