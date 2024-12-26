.DEFAULT_GOAL := all
sources = python/pymdps tests

.PHONY: clean
clean:
	rm -rf `find . -name __pycache__`
	rm -f `find . -type f -name '*.py[co]' `
	rm -f `find . -type f -name '*~' `
	rm -f `find . -type f -name '.*~' `
	rm -rf .cache
	rm -rf htmlcov
	rm -rf .pytest_cache
	rm -rf *.egg-info
	rm -f .coverage
	rm -f .coverage.*
	rm -rf build
	rm -rf target
	rm -rf perf.data*
	rm -rf python/pymdps/*.so

.PHONY: install-dev
install-dev: clean
	pip install -e ".[all]"

.PHONY: install
install: clean
	pip install -e .

.PHONY: test
test: install-dev
	pytest -v --cov=python/pymdps --cov-report=term-missing --cov-report=html tests

.PHONY: docs
docs: install-dev
	(cd docs && mkdocs build)

.PHONY: serve-docs
serve-docs: install-dev
	(cd docs && mkdocs serve)

.PHONY: all
all: clean test