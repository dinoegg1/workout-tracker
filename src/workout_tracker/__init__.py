import streamlit as st

st.title("Workout Tracker")
st.subheader("Landing Page")
st.caption("Select your device to get started.")
st.page_link("pages/Mobile.py", label="Mobile")
st.page_link("pages/Desktop.py", label="Desktop")
st.page_link("pages/Testing.py", label="Testing")
