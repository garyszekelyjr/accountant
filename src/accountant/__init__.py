from sqlalchemy import create_engine

from accountant.models import Base


ENGINE = create_engine("sqlite:///db.sqlite")

# Base.metadata.drop_all(ENGINE)
Base.metadata.create_all(ENGINE)
