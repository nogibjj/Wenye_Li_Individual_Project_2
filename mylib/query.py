import os
from databricks import sql
from dotenv import load_dotenv
import logging

# Setup logging configuration
logging.basicConfig(
    level=logging.INFO,
    format='**%(asctime)s** - %(levelname)s - %(message)s',
    handlers=[
        logging.FileHandler("db_operations.md"),
        logging.StreamHandler()
    ]
)

load_dotenv()

# Load environment variables
server_h = os.getenv("SERVER_HOSTNAME")
access_token = os.getenv("DATABRICKS_KEY")
http_path = os.getenv("HTTP_PATH")


def log_query(query, result="none"):
    """Logs the query and result in markdown format"""
    logging.info(f"Executed query:\n```sql\n{query}\n```")
    logging.info(f"Query results:\n{result}")


def run_query(query):
    """Executes a user-provided SQL query and logs the results"""
    with sql.connect(
        server_hostname=server_h,
        http_path=http_path,
        access_token=access_token,
    ) as connection:
        c = connection.cursor()
        c.execute(query)
        result = c.fetchall()
        log_query(query, result)
        for row in result:
            print(row)
        c.close()
    return result


# if __name__ == "__main__":
    # user_query = """
    #     WITH AgeStats AS (
    #         SELECT age,
    #                AVG(alcohol_use) AS avg_alcohol_use,
    #                AVG(marijuana_use) AS avg_marijuana_use
    #         FROM DrugUseDB
    #         GROUP BY age
    #     )
    #     SELECT d.age, d.n, d.alcohol_use, a.avg_alcohol_use, 
    #            d.marijuana_use, a.avg_marijuana_use
    #     FROM DrugUseDB d
    #     JOIN AgeStats a
    #     ON d.age = a.age
    #     ORDER BY d.age ASC, d.n DESC
    # """
#     run_query(user_query)