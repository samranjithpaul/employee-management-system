# ================================================================
#  Employee Management System (EMS)
#  Developed by: Sam Ranjith Paul
#  GitHub: https://github.com/samranjithpaul
#  LinkedIn: https://www.linkedin.com/in/Samranjithpaul
#  Unauthorized removal of this header is prohibited.
# ================================================================
"""
Footer utility for displaying developer information
"""
import streamlit as st
from datetime import datetime

def footer():
    """Display footer with developer information"""
    st.markdown(
        f"""
        <hr>
        <div style='text-align: center; font-size: 13px; color: gray; padding: 20px;'>
            © {datetime.now().year} Employee Management System<br>
            Developed by <b>Sam Ranjith Paul</b><br>
            <a href="https://github.com/samranjithpaul" target="_blank" style='color: #1f77b4; text-decoration: none;'>GitHub</a> |
            <a href="https://www.linkedin.com/in/Samranjithpaul" target="_blank" style='color: #1f77b4; text-decoration: none;'>LinkedIn</a>
        </div>
        """,
        unsafe_allow_html=True
    )

def sidebar_branding():
    """Display branding in sidebar"""
    st.sidebar.markdown("---")
    st.sidebar.caption("🧠 EMS v1.0")
    st.sidebar.caption("Built by **Sam Ranjith Paul**")
    st.sidebar.markdown(
        """
        <div style='font-size: 11px; text-align: center; color: gray;'>
            <a href="https://github.com/samranjithpaul" target="_blank" style='color: #1f77b4;'>GitHub</a> |
            <a href="https://www.linkedin.com/in/Samranjithpaul" target="_blank" style='color: #1f77b4;'>LinkedIn</a>
        </div>
        """,
        unsafe_allow_html=True
    )

