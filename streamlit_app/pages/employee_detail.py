# ================================================================
#  Employee Management System (EMS)
#  Developed by: Sam Ranjith Paul
#  GitHub: https://github.com/samranjithpaul
#  LinkedIn: https://www.linkedin.com/in/Samranjithpaul
#  Unauthorized removal of this header is prohibited.
# ================================================================
"""
Employee Detail Page - View employee details, history, and payslips
"""
import streamlit as st
import sys
import os
import tempfile

# Add parent directory to path
sys.path.append(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from utils.api_client import (
    get_employee, get_employment_history, add_employment_history,
    list_payslips, get_payslip_url, upload_payslip
)
from utils.auth import require_login, get_token
from utils.export_utils import export_employee_to_word, export_employee_to_pdf
from utils.footer import footer, sidebar_branding

st.set_page_config(page_title="Employee Details - EMS", page_icon="👤", layout="wide")

require_login()

# Add sidebar branding
sidebar_branding()

token = get_token()

# Get employee ID
emp_id = st.session_state.get("selected_emp_id")

if not emp_id:
    st.error("No employee selected")
    st.stop()

# Get employee data
employee = get_employee(token, emp_id)

if not employee:
    st.error("Employee not found")
    st.stop()

st.title(f"👤 {employee['first_name']} {employee['last_name']}")

# Back button and Export buttons
col1, col2, col3 = st.columns([2, 1, 1])
with col1:
    if st.button("← Back to Dashboard"):
        if "selected_emp_id" in st.session_state:
            del st.session_state["selected_emp_id"]
        st.switch_page("pages/dashboard.py")
with col2:
    if st.button("📝 Export as Word"):
        with st.spinner("Generating Word document..."):
            filepath = export_employee_to_word(emp_id, token)
            if filepath and os.path.exists(filepath):
                with open(filepath, "rb") as f:
                    st.download_button(
                        "⬇️ Download Word",
                        f.read(),
                        file_name=os.path.basename(filepath),
                        mime="application/vnd.openxmlformats-officedocument.wordprocessingml.document",
                        key="download_word"
                    )
with col3:
    if st.button("📄 Export as PDF"):
        with st.spinner("Generating PDF document..."):
            filepath = export_employee_to_pdf(emp_id, token)
            if filepath and os.path.exists(filepath):
                with open(filepath, "rb") as f:
                    st.download_button(
                        "⬇️ Download PDF",
                        f.read(),
                        file_name=os.path.basename(filepath),
                        mime="application/pdf",
                        key="download_pdf"
                    )

st.markdown("---")

# Employee Information
col1, col2 = st.columns(2)

with col1:
    st.subheader("📋 Personal Information")
    st.write(f"**Employee ID:** {employee['emp_id']}")
    st.write(f"**First Name:** {employee['first_name']}")
    st.write(f"**Last Name:** {employee['last_name']}")
    st.write(f"**Email:** {employee.get('email', 'N/A')}")
    st.write(f"**Phone:** {employee.get('phone', 'N/A')}")

with col2:
    st.subheader("💼 Work Information")
    st.write(f"**Department:** {employee.get('department', 'N/A')}")
    st.write(f"**Designation:** {employee.get('designation', 'N/A')}")
    st.write(f"**Status:** {employee['status']}")
    st.write(f"**Joining Date:** {employee.get('joining_date', 'N/A')}")
    st.write(f"**Created At:** {employee.get('created_at', 'N/A')}")

st.markdown("---")

# Tabs for History and Payslips
tab1, tab2 = st.tabs(["📜 Employment History", "💰 Payslips"])

with tab1:
    st.subheader("Previous Employment History")
    
    # Get employment history
    history = get_employment_history(token, emp_id)
    
    if history:
        for item in history:
            with st.expander(f"🏢 {item['company_name']} - {item.get('role', 'N/A')}"):
                col1, col2 = st.columns(2)
                with col1:
                    st.write(f"**Company:** {item['company_name']}")
                    st.write(f"**Role:** {item.get('role', 'N/A')}")
                with col2:
                    st.write(f"**Start Date:** {item.get('start_date', 'N/A')}")
                    st.write(f"**End Date:** {item.get('end_date', 'N/A')}")
    else:
        st.info("No employment history found.")
    
    # Add new employment history
    st.markdown("### ➕ Add Previous Employment")
    with st.form("add_history_form"):
        col1, col2 = st.columns(2)
        
        with col1:
            company_name = st.text_input("Company Name *", placeholder="Tech Corp")
            start_date = st.date_input("Start Date")
        
        with col2:
            role = st.text_input("Role", placeholder="Software Engineer")
            end_date = st.date_input("End Date")
        
        submit_history = st.form_submit_button("Add History", use_container_width=True)
        
        if submit_history:
            if not company_name:
                st.error("Company name is required")
            else:
                history_data = {
                    "company_name": company_name,
                    "role": role if role else None,
                    "start_date": str(start_date) if start_date else None,
                    "end_date": str(end_date) if end_date else None,
                }
                
                with st.spinner("Adding employment history..."):
                    if add_employment_history(token, emp_id, history_data):
                        st.success("Employment history added successfully!")
                        st.rerun()
                    else:
                        st.error("Failed to add employment history")

with tab2:
    st.subheader("Payslip Documents")
    
    # List payslips
    payslips = list_payslips(token, emp_id)
    
    if payslips:
        st.write(f"Found {len(payslips)} payslip(s):")
        for payslip in payslips:
            col1, col2 = st.columns([3, 1])
            with col1:
                # Determine file icon and type based on extension
                filename = payslip['filename']
                file_ext = filename.split('.')[-1].lower() if '.' in filename else 'pdf'
                
                if file_ext in ['xlsx', 'xls']:
                    icon = "📊"
                    file_type = "Excel"
                elif file_ext in ['docx', 'doc']:
                    icon = "📝"
                    file_type = "Word"
                else:
                    icon = "📄"
                    file_type = "PDF"
                
                file_size = payslip.get('size', 0)
                size_mb = file_size / (1024 * 1024) if file_size > 0 else 0
                size_kb = file_size / 1024 if file_size > 0 else 0
                if size_mb >= 1:
                    size_str = f"{size_mb:.2f} MB"
                elif size_kb >= 1:
                    size_str = f"{size_kb:.2f} KB"
                else:
                    size_str = f"{file_size} bytes"
                st.write(f"{icon} **{filename}** ({file_type} - {size_str})")
            with col2:
                payslip_url = get_payslip_url(payslip['filename'])
                st.markdown(f"[⬇️ Download]({payslip_url})")
    else:
        st.info("No payslips uploaded yet.")
    
    # Upload payslip
    st.markdown("### 📤 Upload Payslip")
    uploaded_file = st.file_uploader(
        "Choose a file (PDF, Excel, or Word)",
        type=['pdf', 'xlsx', 'xls', 'docx', 'doc'],
        help="Upload employee payslip as PDF, Excel (.xlsx, .xls), or Word (.docx, .doc)"
    )
    
    if uploaded_file is not None:
        # Get file extension from uploaded file name
        file_extension = uploaded_file.name.split('.')[-1].lower() if '.' in uploaded_file.name else 'pdf'
        
        # Validate file extension
        allowed_extensions = ['pdf', 'xlsx', 'xls', 'docx', 'doc']
        if file_extension not in allowed_extensions:
            st.error(f"Unsupported file type. Allowed: PDF, Excel (.xlsx, .xls), Word (.docx, .doc)")
        elif st.button("Upload Payslip", type="primary"):
            # Save file temporarily with correct extension
            suffix = f'.{file_extension}'
            with tempfile.NamedTemporaryFile(delete=False, suffix=suffix) as tmp_file:
                tmp_file.write(uploaded_file.read())
                tmp_path = tmp_file.name
            
            # Upload to API
            with st.spinner("Uploading payslip..."):
                result = upload_payslip(token, emp_id, tmp_path)
                if result:
                    st.success("Payslip uploaded successfully!")
                    # Clean up temp file
                    try:
                        os.unlink(tmp_path)
                    except:
                        pass
                    st.rerun()
                else:
                    st.error("Failed to upload payslip")
                    try:
                        os.unlink(tmp_path)
                    except:
                        pass


# Display footer
footer()
