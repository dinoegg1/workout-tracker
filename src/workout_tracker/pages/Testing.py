import requests
import streamlit as st
from streamlit import secrets

get_request_url = secrets.REQUEST_URL

st.title("Testing")

if st.button("Test API"):
    response = requests.post(f"{get_request_url}/testing/v1")
    st.write(response.json())
