# Just The Game - Build System

# Use bash for recipes that rely on bashisms
set shell := ["bash", "-c"]

# Default recipe - show available commands
default:
    @just --list

# Use venv-managed tools by default
python := "venv/bin/python3"
pip := "venv/bin/pip"

# Set up Python virtual environment and install dependencies
setup:
    python3 -m venv venv
    @echo "Virtual environment created."
    @echo "Installing dependencies..."
    {{pip}} install -r requirements.txt
    @echo ""
    @echo "Setup complete! Activate the environment with:"
    @echo "  source venv/bin/activate  # On macOS/Linux"
    @echo "  venv\\Scripts\\activate    # On Windows"

# Install Python dependencies
install:
    {{pip}} install -r requirements.txt

# Clean generated files and test output
clean:
    rm -rf test_output/
    @echo "Cleaned test output directory"

# Build the project - bundle everything into index.html
build:
    {{python}} pack_project.py

# Run all tests (builds first, then runs unit tests, then integration tests)
test: build
    RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'; BLUE='\033[0;34m'; NC='\033[0m'; \
    echo -e "$BLUE🚀 Starting Test Runner$NC"; \
    echo "=================================================="; \
    rm -rf test_output; mkdir -p test_output; \
    echo -e "\n$YELLOW📋 Running Unit Tests (Fast)$NC"; \
    echo "--------------------------------------------------"; \
    unit_failed=0; unit_total=0; \
    for test_file in $(find tests -name 'unit_test*.py' -type f | sort); do \
        unit_total=$((unit_total+1)); \
        test_name=$(basename "$test_file" .py); \
        output_file="test_output/$test_name.log"; \
        echo -n "Running $test_name... "; \
        if {{python}} tests/run_one_test.py "$test_file" > "$output_file" 2>&1; then \
            echo -e "$GREEN✅ PASSED$NC"; \
        else \
            echo -e "$RED❌ FAILED$NC"; \
            echo "  See: $output_file"; \
            unit_failed=$((unit_failed+1)); \
        fi; \
    done; \
    echo ""; \
    if [ $unit_failed -ne 0 ]; then \
        echo -e "$RED❌ $unit_failed of $unit_total unit tests failed$NC"; \
        echo -e "$REDStopping here - fix unit tests before running integration tests$NC"; \
        exit 1; \
    else \
        echo -e "$GREEN🎉 All $unit_total unit tests passed!$NC"; \
        echo ""; \
    fi; \
    echo -e "$YELLOW🔧 Running Integration Tests (Slow)$NC"; \
    echo "--------------------------------------------------"; \
    integration_failed=0; integration_total=0; \
    for test_file in $(find tests -name 'integration_test_*.py' -type f | sort); do \
        integration_total=$((integration_total+1)); \
        test_name=$(basename "$test_file" .py); \
        output_file="test_output/$test_name.log"; \
        echo -n "Running $test_name... "; \
        if {{python}} tests/run_one_test.py "$test_file" > "$output_file" 2>&1; then \
            echo -e "$GREEN✅ PASSED$NC"; \
        else \
            echo -e "$RED❌ FAILED$NC"; \
            echo "  See: $output_file"; \
            integration_failed=$((integration_failed+1)); \
        fi; \
    done; \
    echo ""; \
    echo "=================================================="; \
    echo -e "$BLUE📊 Test Summary$NC"; \
    echo "=================================================="; \
    echo -e "Unit Tests:        $GREEN$((unit_total - unit_failed))/$unit_total passed$NC"; \
    if [ $integration_total -gt 0 ]; then \
        if [ $integration_failed -eq 0 ]; then \
            echo -e "Integration Tests: $GREEN$((integration_total - integration_failed))/$integration_total passed$NC"; \
        else \
            echo -e "Integration Tests: $RED$((integration_total - integration_failed))/$integration_total passed$NC"; \
        fi; \
    fi; \
    if [ $integration_failed -eq 0 ]; then \
        echo -e "\n$GREEN🎉 All tests passed!$NC"; \
        exit 0; \
    else \
        echo -e "\n$RED❌ Some integration tests failed$NC"; \
        exit 1; \
    fi

# Run only unit tests
test-unit: build
    @echo "Running unit tests..."
    @for test in tests/unit_test*.py; do \
        if [ -f "$test" ]; then \
            echo -n "Running $(basename $test)... "; \
            if {{python}} tests/run_one_test.py "$test" > /dev/null 2>&1; then \
                echo "✅ PASSED"; \
            else \
                echo "❌ FAILED"; \
                exit 1; \
            fi; \
        fi; \
    done

# Run only integration tests
test-integration: build
    @echo "Running integration tests..."
    @for test in tests/integration_test_*.py; do \
        if [ -f "$test" ]; then \
            echo -n "Running $(basename $test)... "; \
            if {{python}} tests/run_one_test.py "$test" > /dev/null 2>&1; then \
                echo "✅ PASSED"; \
            else \
                echo "❌ FAILED"; \
            fi; \
        fi; \
    done

# Run a single test file
test-one FILE: build
    {{python}} tests/run_one_test.py {{FILE}}

# Verify build info
verify: build
    {{python}} test_build_info.py

# Start a simple HTTP server for local testing
serve: build
    @echo "Starting server at http://localhost:8000"
    {{python}} -m http.server 8000

# Run build and start server
dev: build serve