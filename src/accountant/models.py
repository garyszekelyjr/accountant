from datetime import date

from typing import List

from sqlalchemy import ForeignKey
from sqlalchemy.orm import DeclarativeBase, Mapped, mapped_column, relationship


class Base(DeclarativeBase):
    pass


class Issuer(Base):
    __tablename__ = "issuer"

    id: Mapped[int] = mapped_column(primary_key=True)
    name: Mapped[str] = mapped_column(unique=True)
    bills: Mapped[List["Bill"]] = relationship(back_populates="issuer")


class Bill(Base):
    __tablename__ = "bill"

    id: Mapped[int] = mapped_column(primary_key=True)
    issuer_id: Mapped[int] = mapped_column(ForeignKey("issuer.id"))
    issuer: Mapped[Issuer] = relationship(back_populates="bills")
    issue_date: Mapped[date] = mapped_column()
    due_date: Mapped[date] = mapped_column()
    amount: Mapped[int] = mapped_column()
