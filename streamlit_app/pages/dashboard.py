# ================================================================
#  Employee Management System (EMS)
#  Developed by: Sam Ranjith Paul
#  GitHub: https://github.com/samranjithpaul
#  LinkedIn: https://www.linkedin.com/in/Samranjithpaul
#  Unauthorized removal of this header is prohibited.
# ================================================================
"""
Dashboard Page - View and manage all employees
"""
import streamlit as st
import sys
import os
import pandas as pd
from datetime import datetime

# Add parent directory to path
sys.path.append(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from utils.api_client import get_employees, delete_employee
from utils.auth import require_login, logout, get_token
from utils.footer import footer, sidebar_branding

st.set_page_config(page_title="Dashboard - EMS", page_icon="📊", layout="wide")

require_login()

# Add sidebar branding
sidebar_branding()

st.title("📊 Employee Dashboard")
st.markdown("---")

# Header with navigation
col1, col2, col3 = st.columns([4, 1, 1])
with col1:
    st.write(f"Welcome, **{st.session_state.get('email', 'Admin')}**")
with col2:
    if st.button("📤 Export Data", type="secondary", use_container_width=True):
        st.switch_page("pages/export_data.py")
with col3:
    if st.button("Logout", type="secondary", use_container_width=True):
        logout()
        st.rerun()

# Get all employees
token = get_token()
employees = get_employees(token)

if not employees:
    st.info("No employees found. Add your first employee!")
    if st.button("Add Employee", type="primary"):
        st.switch_page("pages/add_employee.py")
else:
    # Convert to DataFrame
    df = pd.DataFrame(employees)
    
    # Sidebar filters
    st.sidebar.header("🔍 Filters")
    
    # Department filter
    departments = ["All"] + sorted([d for d in df["department"].dropna().unique() if d])
    selected_department = st.sidebar.selectbox("Department", departments)
    
    # Designation filter
    designations = ["All"] + sorted([d for d in df["designation"].dropna().unique() if d])
    selected_designation = st.sidebar.selectbox("Designation", designations)
    
    # Status filter
    statuses = ["All", "Active", "Inactive"]
    selected_status = st.sidebar.selectbox("Status", statuses)
    
    # Search bar
    search_query = st.sidebar.text_input("Search (Name, Email, Phone)")
    
    # Apply filters
    filtered_df = df.copy()
    
    if selected_department != "All":
        filtered_df = filtered_df[filtered_df["department"] == selected_department]
    
    if selected_designation != "All":
        filtered_df = filtered_df[filtered_df["designation"] == selected_designation]
    
    if selected_status != "All":
        filtered_df = filtered_df[filtered_df["status"] == selected_status]
    
    if search_query:
        mask = (
            filtered_df["first_name"].str.contains(search_query, case=False, na=False) |
            filtered_df["last_name"].str.contains(search_query, case=False, na=False) |
            filtered_df["email"].str.contains(search_query, case=False, na=False) |
            filtered_df["phone"].str.contains(search_query, case=False, na=False)
        )
        filtered_df = filtered_df[mask]
    
    # Display statistics
    col1, col2, col3, col4 = st.columns(4)
    with col1:
        st.metric("Total Employees", len(df))
    with col2:
        st.metric("Active Employees", len(df[df["status"] == "Active"]))
    with col3:
        st.metric("Departments", df["department"].nunique())
    with col4:
        st.metric("Filtered Results", len(filtered_df))
    
    st.markdown("---")
    
    # Action buttons
    col1, col2 = st.columns([1, 5])
    with col1:
        if st.button("➕ Add Employee", type="primary", use_container_width=True):
            st.switch_page("pages/add_employee.py")
    
    # Display employees table
    if len(filtered_df) > 0:
        # Format the dataframe for display
        display_df = filtered_df[[
            "emp_id", "first_name", "last_name", "email", 
            "phone", "department", "designation", "status"
        ]].copy()
        display_df.columns = ["ID", "First Name", "Last Name", "Email", "Phone", "Department", "Designation", "Status"]
        
        # Create action buttons for each row
        for idx, row in filtered_df.iterrows():
            with st.expander(f"👤 {row['first_name']} {row['last_name']} - {row.get('designation', 'N/A')}"):
                col1, col2, col3, col4 = st.columns(4)
                
                with col1:
                    st.write(f"**ID:** {row['emp_id']}")
                    st.write(f"**Email:** {row.get('email', 'N/A')}")
                    st.write(f"**Phone:** {row.get('phone', 'N/A')}")
                
                with col2:
                    st.write(f"**Department:** {row.get('department', 'N/A')}")
                    st.write(f"**Designation:** {row.get('designation', 'N/A')}")
                    st.write(f"**Status:** {row['status']}")
                
                with col3:
                    if row.get('joining_date'):
                        st.write(f"**Joining Date:** {row['joining_date']}")
                    st.write(f"**Created:** {row.get('created_at', 'N/A')}")
                
                with col4:
                    if st.button("👁️ View Details", key=f"view_{row['emp_id']}", use_container_width=True):
                        st.session_state["selected_emp_id"] = row['emp_id']
                        st.switch_page("pages/employee_detail.py")
                    
                    if st.button("✏️ Edit", key=f"edit_{row['emp_id']}", use_container_width=True):
                        st.session_state["edit_emp_id"] = row['emp_id']
                        st.switch_page("pages/add_employee.py")
                    
                    if st.button("🗑️ Delete", key=f"delete_{row['emp_id']}", use_container_width=True):
                        if delete_employee(token, row['emp_id']):
                            st.success("Employee deleted successfully!")
                            st.rerun()
                        else:
                            st.error("Failed to delete employee")
    else:
        st.info("No employees match the selected filters.")

