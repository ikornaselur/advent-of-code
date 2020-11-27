mypy:
	@poetry run mypy src/advent_of_code/*

flake8:
	@poetry run flake8 src/advent_of_code/*

lint: mypy flake8

shell:
	@poetry run ipython

test: unit_test

unit_test:
	@PYTHONPATH=src poetry run pytest tests/unit -xvvs

install_git_hooks:
	@ln -s `pwd`/.hooks/pre-push .git/hooks/pre-push

run:
	@PYTHONPATH=src poetry run python run_all.py
