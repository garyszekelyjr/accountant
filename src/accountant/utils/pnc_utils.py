import pandas as pd


def calculate_balance(df) -> pd.Series:
    df["amount"] = df["amount"].where(df["category"] == "Deposits and Other Additions", -df["amount"])

    df["date"] = pd.to_datetime(df["date"])
    df["year"] = df["date"].dt.year
    df["month"] = df["date"].dt.month

    return df["amount"].cumsum().groupby([df["year"], df["month"]]).last()


def calculate_categories(df) -> pd.DataFrame:
    df = df[df["category"] == "Banking/Debit Card Withdrawals and Purchases"]

    df["date"] = pd.to_datetime(df["date"])
    df["year"] = df["date"].dt.year
    df["month"] = df["date"].dt.month

    return df.groupby(["year", "month"])["amount"].sum()
