from sqlalchemy.orm import Session

from accountant import ENGINE
from accountant.targs import TArgs
from accountant.utils.capital_one_scraper import scrape
from accountant.utils.utils import statement_to_bill

args = TArgs().parse_args()

ENGINE.echo = args.verbose

with Session(ENGINE) as session:
    match " ".join(args.command):
        case "statement-to-bill":
            statement_to_bill(session)
        case "scrape-capital-one":
            scrape()
        case _:
            print("Invalid Command")
