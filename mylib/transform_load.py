from databricks import sql
import csv
import os
from dotenv import load_dotenv

def load(dataset="data/drug-use-by-age.csv"):
    """Transforms and Loads data into the external Databricks database"""

    print(os.getcwd())
    
    with open(dataset, newline='', encoding='utf-8') as csvfile:
        reader = csv.DictReader(csvfile)

        load_dotenv()
        server_h = os.getenv("SERVER_HOSTNAME")
        access_token = os.getenv("DATABRICKS_KEY")
        http_path = os.getenv("HTTP_PATH")

        with sql.connect(
            server_hostname=server_h,
            http_path=http_path,
            access_token=access_token,
        ) as connection:
            c = connection.cursor()

            c.execute("DROP TABLE IF EXISTS DrugUseDB")

            c.execute("""
                CREATE TABLE IF NOT EXISTS DrugUseDB (
                    age STRING,
                    n INT,
                    alcohol_use FLOAT,
                    alcohol_frequency FLOAT,
                    marijuana_use FLOAT,
                    marijuana_frequency FLOAT,
                    cocaine_use FLOAT,
                    cocaine_frequency FLOAT,
                    crack_use FLOAT,
                    crack_frequency FLOAT,
                    heroin_use FLOAT,
                    heroin_frequency FLOAT,
                    hallucinogen_use FLOAT,
                    hallucinogen_frequency FLOAT,
                    inhalant_use FLOAT,
                    inhalant_frequency FLOAT,
                    pain_releiver_use FLOAT,
                    pain_releiver_frequency FLOAT,
                    oxycontin_use FLOAT,
                    oxycontin_frequency FLOAT,
                    tranquilizer_use FLOAT,
                    tranquilizer_frequency FLOAT,
                    stimulant_use FLOAT,
                    stimulant_frequency FLOAT,
                    meth_use FLOAT,
                    meth_frequency FLOAT,
                    sedative_use FLOAT,
                    sedative_frequency FLOAT
                )
            """)

            data = []
            for row in reader:
                data.append((
                    row['age'],
                    int(row['n']),
                    float(row['alcohol_use']),
                    float(row['alcohol_frequency']),
                    float(row['marijuana_use']),
                    float(row['marijuana_frequency']),
                    float(row['cocaine_use']),
                    float(row['cocaine_frequency']) 
                    if row['cocaine_frequency'] != '-' else None,
                    float(row['crack_use']) 
                    if row['crack_use'] != '-' else None,
                    float(row['crack_frequency']) 
                    if row['crack_frequency'] != '-' else None,
                    float(row['heroin_use']) 
                    if row['heroin_use'] != '-' else None,
                    float(row['heroin_frequency']) 
                    if row['heroin_frequency'] != '-' else None,
                    float(row['hallucinogen_use']),
                    float(row['hallucinogen_frequency']),
                    float(row['inhalant_use']),
                    float(row['inhalant_frequency']) 
                    if row['inhalant_frequency'] != '-' else None,
                    float(row['pain_releiver_use']),
                    float(row['pain_releiver_frequency']),
                    float(row['oxycontin_use']),
                    float(row['oxycontin_frequency']) 
                    if row['oxycontin_frequency'] != '-' else None,
                    float(row['tranquilizer_use']),
                    float(row['tranquilizer_frequency']),
                    float(row['stimulant_use']),
                    float(row['stimulant_frequency']),
                    float(row['meth_use']) 
                    if row['meth_use'] != '-' else None,
                    float(row['meth_frequency']) 
                    if row['meth_frequency'] != '-' else None,
                    float(row['sedative_use']),
                    float(row['sedative_frequency'])
                ))

            c.executemany("""
                INSERT INTO DrugUseDB VALUES (
                    ?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?
                )
            """, data)

            connection.commit()
            c.close()
    
    return "Load success"

if __name__ == "__main__":
    load()