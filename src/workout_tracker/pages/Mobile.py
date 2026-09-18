import datetime  # noqa: I001

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
    try:
        # Convert Pydantic model to dict and ensure date is a string for JSON serialization
        workout_data = workout.dict() if hasattr(workout, 'dict') else workout.model_dump()
        workout_data['date'] = workout_data['date'].isoformat()

        # Using the 'json=' parameter automatically sets Content-Type to application/json
        response = requests.post(f"{get_request_url}/workout/v1", json=workout_data)

        if response.status_code == 200:
            st.write("Response Saved!")
            st.write(response.json())
        else:
            st.error(f"Failed to save. Status: {response.status_code}")
            st.write(f"Server said: {response.text}")

    except requests.exceptions.RequestException as e:
        st.error(f"{e}")
        if 'response' in locals():
            st.write(f"Server said: {response.text}")
            st.write(f"Response status: {response.status_code}")
            st.write(f"Response headers: {response.headers}")
