.PHONY: check deps fmt lint vet deny test bench coverage build distlib doc clean

# Local cleanliness + test gate (deps + fmt + lint + build + full build + tests)
check:
	sh scripts/check.sh

# Resolve stdlib + git deps into lib/
deps:
	cyrius deps

# Format check
fmt:
	cyrius fmt src/main.cyr --check

# Static analysis (zero warnings)
lint:
	cyrius lint src/main.cyr

# Audit include dependencies
vet:
	cyrius vet src/main.cyr

# Enforce project policies
deny:
	cyrius deny src/main.cyr

# Run test suite (tests/*.tcyr)
test:
	cyrius tests

# Run benchmarks with history
bench:
	./scripts/bench-history.sh

# Generate coverage report
coverage:
	cyrius coverage

# Build the engine + demo entry (all optional surfaces)
build:
	cyrius build -D LOGGING -D MCP -D DAIMON -D HOOSH src/main.cyr build/varna

# Bundle src/ into the consumer distfile
distlib:
	cyrius distlib

# Generate / check documentation
doc:
	cyrius doc --check src/main.cyr

# Clean build artifacts
clean:
	cyrius clean
