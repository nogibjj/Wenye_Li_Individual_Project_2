[![CI](https://github.com/nogibjj/Wenye_Li_Mini_Project_7/actions/workflows/cicd.yml/badge.svg)](https://github.com/nogibjj/Wenye_Li_Mini_Project_7/actions/workflows/cicd.yml)

## Wenye Li Mini Project 7

## Requirements

- Package a Python script with setuptools or a similar tool
- Include a user guide on how to install and use the tool
- Include communication with an external or internal database (NoSQL, SQL, etc)

## Deliverables

- Packaged tool
- Written Guide

## Project Structures

- `devcontainer`

- `cicd.yml`

- `mylib`:

  - **extract.py**
  - **transform_load**
  - **queryt**

- `Makefile`:

- `requirements.txt`

- `setup.py`

- `main.py`

- `test_main.py`

- `README.me`

- `db_operations.md` (please check this for logging info)

## Preparation

1. Built virtual environment: run `make install`
2. build packaged project by running `make setup_package`
3. extract: run `make extract`
4. transform and load: run `make transform_load`
5. query: run `make query`

## Dataset

| Header                    | Definition                                                                              |
| ------------------------- | --------------------------------------------------------------------------------------- |
| `alcohol-use`             | Percentage of those in an age group who used alcohol in the past 12 months              |
| `alcohol-frequency`       | Median number of times a user in an age group used alcohol in the past 12 months        |
| `marijuana-use`           | Percentage of those in an age group who used marijuana in the past 12 months            |
| `marijuana-frequency`     | Median number of times a user in an age group used marijuana in the past 12 months      |
| `cocaine-use`             | Percentage of those in an age group who used cocaine in the past 12 months              |
| `cocaine-frequency`       | Median number of times a user in an age group used cocaine in the past 12 months        |
| `crack-use`               | Percentage of those in an age group who used crack in the past 12 months                |
| `crack-frequency`         | Median number of times a user in an age group used crack in the past 12 months          |
| `heroin-use`              | Percentage of those in an age group who used heroin in the past 12 months               |
| `heroin-frequency`        | Median number of times a user in an age group used heroin in the past 12 months         |
| `hallucinogen-use`        | Percentage of those in an age group who used hallucinogens in the past 12 months        |
| `hallucinogen-frequency`  | Median number of times a user in an age group used hallucinogens in the past 12 months  |
| `inhalant-use`            | Percentage of those in an age group who used inhalants in the past 12 months            |
| `inhalant-frequency`      | Median number of times a user in an age group used inhalants in the past 12 months      |
| `pain-releiver-use`       | Percentage of those in an age group who used pain relievers in the past 12 months       |
| `pain-releiver-frequency` | Median number of times a user in an age group used pain relievers in the past 12 months |
| `oxycontin-use`           | Percentage of those in an age group who used oxycontin in the past 12 months            |
| `oxycontin-frequency`     | Median number of times a user in an age group used oxycontin in the past 12 months      |
| `tranquilizer-use`        | Percentage of those in an age group who used tranquilizer in the past 12 months         |
| `tranquilizer-frequency`  | Median number of times a user in an age group used tranquilizer in the past 12 months   |
| `stimulant-use`           | Percentage of those in an age group who used stimulants in the past 12 months           |
| `stimulant-frequency`     | Median number of times a user in an age group used stimulants in the past 12 months     |
| `meth-use`                | Percentage of those in an age group who used meth in the past 12 months                 |
| `meth-frequency`          | Median number of times a user in an age group used meth in the past 12 months           |
| `sedative-use`            | Percentage of those in an age group who used sedatives in the past 12 months            |
| `sedative-frequency`      | Median number of times a user in an age group used sedatives in the past 12 months      |

### SQL Query & Result

```python
WITH AgeStats AS (
                SELECT age,
                       AVG(alcohol_use) AS avg_alcohol_use,
                       AVG(marijuana_use) AS avg_marijuana_use
                FROM DrugUseDB
                GROUP BY age
            )
            SELECT d.age, d.n, d.alcohol_use, a.avg_alcohol_use,
            d.marijuana_use, a.avg_marijuana_use
            FROM DrugUseDB d
            JOIN AgeStats a
            ON d.age = a.age
            ORDER BY d.age ASC, d.n DESC
```

### Query Explaination

- Common Table Expression (CTE) - AgeStats: A CTE is created to compute the average use of alcohol and marijuana for each age group using the AVG() function. The dataset is grouped by age, and the average of alcohol_use and marijuana_use is calculated for each age group.

- Main Query: The main query selects individual records from DrugUseDB and joins them with the AgeStats CTE on the age field. This allows for a comparison of the specific alcohol and marijuana use for each individual record with the overall age group’s average alcohol and marijuana use. The result is ordered by age in ascending order, and within each age group, the records are ordered by the number of individuals (n) in descending order.

- Aggregation: AVG(alcohol_use) calculates the average percentage of alcohol use for each age group. And AVG(marijuana_use) calculates the average percentage of marijuana use for each age group.

- Sorting: The output is sorted by age in ascending order (youngest to oldest). For each age group, the results are further sorted by n (the number of individuals) in descending order.

### Result Interpretation

The results of the query provide insights into the relationship between age groups and their average use of alcohol and marijuana.

Key findings include:

- Alcohol and marijuana use both increase with age, peaking in early adulthood, with alcohol consistently more prevalent than marijuana across all age groups.
- Substance use declines sharply in older age groups, particularly for marijuana, which drops significantly after age 25.
