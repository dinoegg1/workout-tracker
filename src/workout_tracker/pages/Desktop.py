import datetime

import pydantic
import requests
import streamlit as st
from streamlit import secrets

get_request_url = secrets.REQUEST_URL

class Workout(pydantic.BaseModel):
    workout_type: str
    amount: int
    date: datetime.date

st.title("Desktop")
st.subheader("To be added later, current work in progress.")

workout_type = st.text_input("Please select the workout type you wish to analyze")

if st.button("Analyze"):
    try:
        response = requests.get(f"{get_request_url}/previous_workout", json={"workout_type": workout_type})
        if response.status_code == 200:
            workouts = response.json()
            st.write(workouts)
        else:
            st.error("Failed to fetch workouts.")
    except requests.exceptions.RequestException as e:
        st.error(f"An error occurred: {e}")
