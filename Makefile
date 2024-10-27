install:
	pip install --upgrade pip &&\
		pip install -r requirements.txt

test:
	python -m pytest -vv --cov=main --cov=mylib test_*.py

format:	
	black *.py 

lint:
	#disable comment to test speed
	#pylint --disable=R,C --ignore-patterns=test_.*?py *.py mylib/*.py
	#ruff linting is 10-100X faster than pylint
	ruff check *.py mylib/*.py

container-lint:
	docker run --rm -i hadolint/hadolint < Dockerfile

refactor: format lint

all: install lint test format

generate_and_push:
	# Create the markdown file 
	python test_main.py  # Replace with the actual command to generate the markdown

	# Add, commit, and push the generated files to GitHub
	@if [ -n "$$(git status --porcelain)" ]; then \
		git config --local user.email "action@github.com"; \
		git config --local user.name "GitHub Action"; \
		git add .; \
		git commit -m "Add SQL log"; \
		git push; \
	else \
		echo "No changes to commit. Skipping commit and push."; \
	fi

extract:
	etl extract

transform_load: 
	etl transform_load

query:
	etl run_query 'WITH AgeStats AS (SELECT age, AVG(alcohol_use) AS avg_alcohol_use, AVG(marijuana_use) AS avg_marijuana_use FROM DrugUseDB GROUP BY age) SELECT d.age, d.n, d.alcohol_use, a.avg_alcohol_use, d.marijuana_use, a.avg_marijuana_use FROM DrugUseDB d JOIN AgeStats a ON d.age = a.age ORDER BY d.age ASC, d.n DESC;'

setup_package: 
	python setup.py develop --user