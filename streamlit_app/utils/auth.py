# ================================================================
#  Employee Management System (EMS)
#  Developed by: Sam Ranjith Paul
#  GitHub: https://github.com/samranjithpaul
#  LinkedIn: https://www.linkedin.com/in/Samranjithpaul
#  Unauthorized removal of this header is prohibited.
# ================================================================
"""
Authentication utilities for Streamlit
"""
import streamlit as st

def require_login():
    """
    Check if user is logged in. If not, stop execution and show error.
    """
    if "token" not in st.session_state:
        st.error("Please log in first.")
        st.stop()

def is_logged_in() -> bool:
    """Check if user is logged in"""
    return "token" in st.session_state

def logout():
    """Clear session and log out user"""
    for key in list(st.session_state.keys()):
        del st.session_state[key]

def get_token() -> str:
    """Get the current JWT token"""
    return st.session_state.get("token", "")

