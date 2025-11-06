# ================================================================
#  Employee Management System (EMS)
#  Developed by: Sam Ranjith Paul
#  GitHub: https://github.com/samranjithpaul
#  LinkedIn: https://www.linkedin.com/in/Samranjithpaul
#  Unauthorized removal of this header is prohibited.
# ================================================================
"""
Main Streamlit App - Employee Management System
"""
import streamlit as st
import sys
import os

# Add utils to path
sys.path.append(os.path.dirname(os.path.abspath(__file__)))

from utils.auth import is_logged_in
from utils.credit_lock import verify_signature

# Verify developer credit integrity
verify_signature()

st.set_page_config(
    page_title="Employee Management System",
    page_icon="🏢",
    layout="wide",
    initial_sidebar_state="expanded"
)

# Check if user is logged in
if not is_logged_in():
    st.switch_page("pages/login.py")
else:
    st.switch_page("pages/dashboard.py")

