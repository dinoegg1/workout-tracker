import datetime

import pydantic
import requests
import streamlit as st
from streamlit import secrets
import json

get_request_url = secrets.REQUEST_URL

class Workout(pydantic.BaseModel):
    workout_type: str
    amount: int
    date: datetime.date

workout_type = st.text_input("Workout Type")
workout_amount = st.slider("How many reps did you do?", 0, 25)
workout_date = st.date_input("Workout Date")

if st.button("Save") and workout_type is not None and workout_amount is not None and workout_date is not None:
    workout = Workout(workout_type=workout_type, amount=workout_amount, date=workout_date)
    requests.post(f"{get_request_url}/workout/v1", json.dumps(Workout,default=str))
