# ================================================================
#  Employee Management System (EMS)
#  Developed by: Sam Ranjith Paul
#  GitHub: https://github.com/samranjithpaul
#  LinkedIn: https://www.linkedin.com/in/Samranjithpaul
#  Unauthorized removal of this header is prohibited.
# ================================================================
"""
Add/Edit Employee Page
"""
import streamlit as st
import sys
import os
from datetime import datetime

# Add parent directory to path
sys.path.append(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from utils.api_client import create_employee, update_employee, get_employee
from utils.auth import require_login, get_token
from utils.footer import footer, sidebar_branding

st.set_page_config(page_title="Add Employee - EMS", page_icon="➕", layout="centered")

require_login()

# Add sidebar branding
sidebar_branding()

token = get_token()

# Check if editing
edit_mode = "edit_emp_id" in st.session_state
emp_id = st.session_state.get("edit_emp_id")

if edit_mode and emp_id:
    st.title("✏️ Edit Employee")
    employee = get_employee(token, emp_id)
    if not employee:
        st.error("Employee not found")
        st.stop()
else:
    st.title("➕ Add New Employee")
    employee = None

st.markdown("---")

with st.form("employee_form"):
    col1, col2 = st.columns(2)
    
    with col1:
        first_name = st.text_input(
            "First Name *",
            value=employee.get("first_name", "") if employee else "",
            placeholder="John"
        )
        email = st.text_input(
            "Email",
            value=employee.get("email", "") if employee else "",
            placeholder="john.doe@company.com"
        )
        department = st.text_input(
            "Department",
            value=employee.get("department", "") if employee else "",
            placeholder="Engineering"
        )
        # Parse joining date if editing
        joining_date_value = None
        if employee and employee.get("joining_date"):
            try:
                # Try parsing different date formats
                date_str = employee["joining_date"]
                if isinstance(date_str, str):
                    # Try ISO format first
                    try:
                        joining_date_value = datetime.fromisoformat(date_str.split('T')[0]).date()
                    except:
                        try:
                            joining_date_value = datetime.strptime(date_str, "%Y-%m-%d").date()
                        except:
                            pass
            except:
                pass
        
        joining_date = st.date_input(
            "Joining Date",
            value=joining_date_value,
            help="Select the employee's joining date"
        )
    
    with col2:
        last_name = st.text_input(
            "Last Name *",
            value=employee.get("last_name", "") if employee else "",
            placeholder="Doe"
        )
        phone = st.text_input(
            "Phone",
            value=employee.get("phone", "") if employee else "",
            placeholder="+1234567890"
        )
        designation = st.text_input(
            "Designation",
            value=employee.get("designation", "") if employee else "",
            placeholder="Software Engineer"
        )
        status = st.selectbox(
            "Status",
            ["Active", "Inactive"],
            index=0 if not employee or employee.get("status") == "Active" else 1
        )
    
    submitted = st.form_submit_button(
        "Save Employee" if edit_mode else "Add Employee",
        use_container_width=True,
        type="primary"
    )
    
    if submitted:
        if not first_name or not last_name:
            st.error("First Name and Last Name are required fields")
        else:
            employee_data = {
                "first_name": first_name,
                "last_name": last_name,
                "email": email if email else None,
                "phone": phone if phone else None,
                "department": department if department else None,
                "designation": designation if designation else None,
                "status": status,
            }
            
            if joining_date:
                employee_data["joining_date"] = str(joining_date)
            
            with st.spinner("Saving employee..."):
                if edit_mode:
                    # Remove None values for update
                    employee_data = {k: v for k, v in employee_data.items() if v is not None}
                    success = update_employee(token, emp_id, employee_data)
                    if success:
                        st.success("Employee updated successfully!")
                        if "edit_emp_id" in st.session_state:
                            del st.session_state["edit_emp_id"]
                        st.rerun()
                    else:
                        st.error("Failed to update employee")
                else:
                    result = create_employee(token, employee_data)
                    if result:
                        st.success("Employee added successfully!")
                        st.rerun()
                    else:
                        st.error("Failed to add employee")

# Back button
if st.button("← Back to Dashboard", use_container_width=True):
    if "edit_emp_id" in st.session_state:
        del st.session_state["edit_emp_id"]
    st.switch_page("pages/dashboard.py")

# Display footer
footer()

