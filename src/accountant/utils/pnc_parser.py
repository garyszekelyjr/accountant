import re
import sys

from datetime import datetime
from pathlib import Path

import pandas as pd
import numpy as np
import textract

from scipy.stats import percentileofscore

categories = {
    "Balance Summary": False,
    "Transaction Summary": False,
    "Interest Summary": False,
    "Deposits and Other Additions": True,
    "Checks and Substitute Checks": True,
    "Banking/Debit Card Withdrawals and Purchases": True,
    "Online and Electronic Banking Deductions": True,
    "Daily Balance Detail": False,
}

linebreak_p = re.compile(r"\r|\n|\x0c")
date_p = re.compile(r"\d{2}\/\d{2}")

blocklist_value_regexes = [
    re.compile(r"There were \d+ other Banking"),
    re.compile(r"Machine/Debit Card deductions\n?"),
    re.compile(r"totaling"),
    re.compile(r"\d+\.\d\d\.\n?"),
]


def rm_custom_chars(txt, row_starts_in_colIdx1=False):
    match = re.match("   ", txt)
    if match:
        if match.start() == 0:
            if row_starts_in_colIdx1:
                txt = re.sub(" ", "_", txt, count=1)

    return txt.replace(",", "").replace("$", " ").strip()


def parse_statement(statement: Path):
    year = int(statement.stem.split("-")[0])
    dfs = []
    text = textract.process(statement, method="pdftotext", layout=True).decode()
    lines = linebreak_p.split(text)
    leading_space_cnt = pd.Series([len(line) - len(line.strip()) for line in lines])
    leading_space_cnt_percs = leading_space_cnt.apply(
        lambda x: percentileofscore(leading_space_cnt.values, x)
    )
    leading_space_cnt_50thPerc = np.percentile(leading_space_cnt.values, 50.0)
    rows_that_start_in_colIdx1 = pd.Series(leading_space_cnt.index).apply(
        lambda x: leading_space_cnt_percs[x] >= 90.0
        and leading_space_cnt[x] > 3 * leading_space_cnt_50thPerc
    )
    rows_that_start_in_colIdx1 = leading_space_cnt[rows_that_start_in_colIdx1].index.tolist()

    new_lines = []
    for i, line in enumerate(lines):
        if not linebreak_p.match(line) and len(line) > 0 and "page" not in line.lower():
            row_starts_in_colIdx1 = False
            if str(i) in rows_that_start_in_colIdx1:
                row_starts_in_colIdx1 = True
            if i + 1 < len(lines):
                if lines[i + 1].startswith("                   "):
                    line = " ".join([line, lines[i + 1].strip()])
            new_lines.append(rm_custom_chars(line, row_starts_in_colIdx1=row_starts_in_colIdx1))
    current_category = ""
    rows = []
    period_found = False
    for _, line in enumerate(new_lines):
        if "For the period" in line and not period_found:
            period = re.search(r"\d{2}\/\d{2}\/\d{4} to \d{2}\/\d{2}\/\d{4}", line)
            if period:
                period = line[period.start() : period.end()]
                start = datetime.strptime(period.split(" to ")[0], "%m/%d/%Y")
                end = datetime.strptime(period.split(" to ")[-1], "%m/%d/%Y")
                period_found = True
                continue

        if not period_found:
            continue

        for category in categories:
            if line.startswith(category):
                current_category = category

        if current_category in categories:
            if categories[current_category]:
                if re.match(date_p, line.strip()):
                    line = "         ".join([line, current_category])
                    values = line.split("  ")
                    row = {}
                    remaining_values = []
                    for val_idx, value in enumerate(values):
                        if val_idx == 0:
                            month = int(value.split("/")[0])
                            day = int(value.split("/")[1])

                            if start.year == end.year:
                                row["date"] = datetime(year=year, month=month, day=day)
                            else:
                                if month == 1:
                                    date_year = end.year
                                elif month == 12:
                                    date_year = start.year
                                row["date"] = datetime(year=date_year, month=month, day=day)
                            continue
                        try:
                            if "amount" not in row:
                                row["amount"] = float(value)
                                continue
                        except Exception:
                            pass

                        if val_idx == len(values) - 1:
                            row["category"] = value.strip()
                            continue

                        for regex in blocklist_value_regexes:
                            value = re.sub(regex, "", value).strip()

                        if bool(value):
                            remaining_values.append(value)

                    row["description"] = " ".join([r.strip() for r in remaining_values])
                    rows.append(row)

        df = pd.DataFrame(rows)
        dfs.append(df)

    df = pd.concat(dfs, ignore_index=True, sort=False)

    if not df.empty:
        df.sort_values(by="date", ascending=True, inplace=True)
        # mask_keep = df["date"].apply(lambda x: x.year == year)
        # df = df[mask_keep].copy()
        df.drop_duplicates(inplace=True)

    return df


def parse_account(account: str):
    return pd.concat(
        [parse_statement(statement) for statement in Path(f"statements/{account}").glob("*.pdf")]
    ).reset_index()
