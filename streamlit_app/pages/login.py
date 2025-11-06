# ================================================================
#  Employee Management System (EMS)
#  Developed by: Sam Ranjith Paul
#  GitHub: https://github.com/samranjithpaul
#  LinkedIn: https://www.linkedin.com/in/Samranjithpaul
#  Unauthorized removal of this header is prohibited.
# ================================================================
"""
Login Page for Employee Management System
"""
import streamlit as st
import sys
import os

# Add parent directory to path
sys.path.append(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from utils.api_client import login, register
from utils.auth import is_logged_in
from utils.footer import footer, sidebar_branding

st.set_page_config(page_title="Login - EMS", page_icon="🔐", layout="centered")

# Add sidebar branding
sidebar_branding()

# Redirect if already logged in
if is_logged_in():
    st.switch_page("pages/dashboard.py")

st.title("🔐 Employee Management System")
st.subheader("Admin Login")

# Create tabs for Login and Register
tab1, tab2 = st.tabs(["Login", "Register"])

with tab1:
    st.markdown("### Login to your account")
    
    with st.form("login_form"):
        email = st.text_input("Email", placeholder="admin@ems.com")
        password = st.text_input("Password", type="password", placeholder="Enter your password")
        submit = st.form_submit_button("Login", use_container_width=True)
        
        if submit:
            if not email or not password:
                st.error("Please fill in all fields")
            else:
                with st.spinner("Logging in..."):
                    token = login(email, password)
                    if token:
                        st.session_state["token"] = token
                        st.session_state["email"] = email
                        st.success("Login successful! Redirecting...")
                        st.rerun()
                    else:
                        st.error("Invalid email or password")

with tab2:
    st.markdown("### Create a new admin account")
    
    with st.form("register_form"):
        reg_email = st.text_input("Email", key="reg_email", placeholder="admin@ems.com")
        reg_password = st.text_input("Password", type="password", key="reg_password", placeholder="Enter password")
        reg_password_confirm = st.text_input("Confirm Password", type="password", key="reg_password_confirm", placeholder="Confirm password")
        submit_register = st.form_submit_button("Register", use_container_width=True)
        
        if submit_register:
            if not reg_email or not reg_password or not reg_password_confirm:
                st.error("Please fill in all fields")
            elif reg_password != reg_password_confirm:
                st.error("Passwords do not match")
            elif len(reg_password) < 6:
                st.error("Password must be at least 6 characters long")
            else:
                with st.spinner("Registering..."):
                    token = register(reg_email, reg_password)
                    if token:
                        st.session_state["token"] = token
                        st.session_state["email"] = reg_email
                        st.success("Registration successful! Redirecting...")
                        st.rerun()
                    else:
                        st.error("Registration failed. Email may already be in use.")

# Display footer
footer()

