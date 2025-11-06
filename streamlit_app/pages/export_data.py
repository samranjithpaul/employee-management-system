# ================================================================
#  Employee Management System (EMS)
#  Developed by: Sam Ranjith Paul
#  GitHub: https://github.com/samranjithpaul
#  LinkedIn: https://www.linkedin.com/in/Samranjithpaul
#  Unauthorized removal of this header is prohibited.
# ================================================================
"""
Data Export Page - Export employee data in various formats
"""
import streamlit as st
import sys
import os
from datetime import datetime

# Add parent directory to path
sys.path.append(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from utils.auth import require_login, get_token
from utils.api_client import get_employees
from utils.export_utils import (
    export_all_to_excel,
    export_all_to_pdf,
    export_employee_to_word,
    export_employee_to_pdf,
    export_all_pdfs_to_zip,
    EXPORTS_DIR
)
from utils.footer import footer, sidebar_branding

st.set_page_config(page_title="Data Export - EMS", page_icon="📤", layout="wide")

require_login()

# Add sidebar branding
sidebar_branding()

st.title("📤 Data Export Center")
st.markdown("---")

token = get_token()

# Sidebar for navigation
st.sidebar.title("Export Options")
export_type = st.sidebar.selectbox(
    "Choose Export Type:",
    [
        "All Employees (Excel)",
        "All Employees (PDF)",
        "All Employees (ZIP Archive)",
        "Single Employee (Word)",
        "Single Employee (PDF)"
    ]
)

st.markdown("### Export Employee Data")

if export_type == "All Employees (Excel)":
    st.info("📊 Export all employee records to an Excel spreadsheet (.xlsx)")
    
    if st.button("⬇️ Generate Excel Export", type="primary", use_container_width=True):
        with st.spinner("Generating Excel file..."):
            filepath = export_all_to_excel(token)
            
            if filepath and os.path.exists(filepath):
                with open(filepath, "rb") as f:
                    st.download_button(
                        "⬇️ Download All Employees (Excel)",
                        f.read(),
                        file_name=os.path.basename(filepath),
                        mime="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
                        use_container_width=True
                    )
                st.success(f"✅ Excel file generated successfully! ({os.path.getsize(filepath)} bytes)")
            else:
                st.error("❌ Failed to generate Excel file. Please try again.")

elif export_type == "All Employees (PDF)":
    st.info("📄 Export all employee records to a single multi-page PDF document")
    
    if st.button("⬇️ Generate PDF Report", type="primary", use_container_width=True):
        with st.spinner("Generating PDF report..."):
            filepath = export_all_to_pdf(token)
            
            if filepath and os.path.exists(filepath):
                with open(filepath, "rb") as f:
                    st.download_button(
                        "⬇️ Download All Employees (PDF)",
                        f.read(),
                        file_name=os.path.basename(filepath),
                        mime="application/pdf",
                        use_container_width=True
                    )
                st.success(f"✅ PDF report generated successfully! ({os.path.getsize(filepath)} bytes)")
            else:
                st.error("❌ Failed to generate PDF report. Please try again.")

elif export_type == "All Employees (ZIP Archive)":
    st.info("📦 Export all employees as individual PDFs bundled in a ZIP archive")
    st.warning("⚠️ This may take a few moments for large employee lists.")
    
    if st.button("⬇️ Generate ZIP Archive", type="primary", use_container_width=True):
        with st.spinner("Generating ZIP archive with all employee PDFs..."):
            filepath = export_all_pdfs_to_zip(token)
            
            if filepath and os.path.exists(filepath):
                with open(filepath, "rb") as f:
                    st.download_button(
                        "⬇️ Download ZIP Archive",
                        f.read(),
                        file_name=os.path.basename(filepath),
                        mime="application/zip",
                        use_container_width=True
                    )
                st.success(f"✅ ZIP archive generated successfully! ({os.path.getsize(filepath)} bytes)")
            else:
                st.error("❌ Failed to generate ZIP archive. Please try again.")

elif export_type == "Single Employee (Word)":
    st.info("📝 Export a single employee's profile as a Word document (.docx)")
    
    # Get list of employees for selection
    employees = get_employees(token)
    
    if employees:
        # Create a selectbox with employee names
        employee_options = {
            f"{emp.get('first_name', '')} {emp.get('last_name', '')} (ID: {emp.get('emp_id')})": emp.get('emp_id')
            for emp in employees
        }
        
        selected_employee = st.selectbox(
            "Select Employee:",
            options=list(employee_options.keys())
        )
        
        emp_id = employee_options.get(selected_employee)
        
        if emp_id and st.button("⬇️ Generate Word Document", type="primary", use_container_width=True):
            with st.spinner("Generating Word document..."):
                filepath = export_employee_to_word(emp_id, token)
                
                if filepath and os.path.exists(filepath):
                    with open(filepath, "rb") as f:
                        st.download_button(
                            "⬇️ Download Word Document",
                            f.read(),
                            file_name=os.path.basename(filepath),
                            mime="application/vnd.openxmlformats-officedocument.wordprocessingml.document",
                            use_container_width=True
                        )
                    st.success(f"✅ Word document generated successfully! ({os.path.getsize(filepath)} bytes)")
                else:
                    st.error("❌ Failed to generate Word document. Please try again.")
    else:
        st.warning("No employees found. Please add employees first.")

elif export_type == "Single Employee (PDF)":
    st.info("📄 Export a single employee's profile as a PDF document")
    
    # Get list of employees for selection
    employees = get_employees(token)
    
    if employees:
        # Create a selectbox with employee names
        employee_options = {
            f"{emp.get('first_name', '')} {emp.get('last_name', '')} (ID: {emp.get('emp_id')})": emp.get('emp_id')
            for emp in employees
        }
        
        selected_employee = st.selectbox(
            "Select Employee:",
            options=list(employee_options.keys())
        )
        
        emp_id = employee_options.get(selected_employee)
        
        if emp_id and st.button("⬇️ Generate PDF Document", type="primary", use_container_width=True):
            with st.spinner("Generating PDF document..."):
                filepath = export_employee_to_pdf(emp_id, token)
                
                if filepath and os.path.exists(filepath):
                    with open(filepath, "rb") as f:
                        st.download_button(
                            "⬇️ Download PDF Document",
                            f.read(),
                            file_name=os.path.basename(filepath),
                            mime="application/pdf",
                            use_container_width=True
                        )
                    st.success(f"✅ PDF document generated successfully! ({os.path.getsize(filepath)} bytes)")
                else:
                    st.error("❌ Failed to generate PDF document. Please try again.")
    else:
        st.warning("No employees found. Please add employees first.")

# Footer with export directory info
st.markdown("---")
with st.expander("ℹ️ Export Information"):
    st.write(f"**Export Directory:** `{EXPORTS_DIR}`")
    st.write("All generated files are saved in the exports directory and can be downloaded using the buttons above.")
    st.write("Files are named with timestamps to prevent overwriting.")

# Display footer
footer()

