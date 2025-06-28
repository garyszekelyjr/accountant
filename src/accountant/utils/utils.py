from datetime import datetime
from pathlib import Path

import questionary
import pandas as pd

from sqlalchemy import select
from sqlalchemy.orm import Session

from accountant.models import Bill, Issuer


def statement_to_bill(session: Session):
    statements = [questionary.Choice(file.stem, file.as_posix()) for file in Path("transactions").glob("*.csv")]
    statements.sort(key=lambda e: str(e.title), reverse=True)

    statement = questionary.select("Statement", choices=statements).ask()

    issuer = session.scalars(select(Issuer).where(Issuer.name == "Capital One")).one()
    issue_date = datetime.strptime(Path(statement).stem, "%Y-%m-%d")

    due_date_year = issue_date.year
    due_date_month = issue_date.month + 1
    due_date_day = 12

    if due_date_month > 12:
        due_date_month -= 12
        due_date_year += 1

    due_date = datetime(due_date_year, due_date_month, due_date_day)

    df = pd.read_csv(statement, index_col="Transaction Date")
    amount = df["Debit"].sum() - df[df["Category"] != "Payment/Credit"]["Credit"].sum()

    bill = Bill(issuer_id=issuer.id, issue_date=issue_date, due_date=due_date, amount=int(amount * 100))
    session.add(bill)

    session.commit()
