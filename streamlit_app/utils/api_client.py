# ================================================================
#  Employee Management System (EMS)
#  Developed by: Sam Ranjith Paul
#  GitHub: https://github.com/samranjithpaul
#  LinkedIn: https://www.linkedin.com/in/Samranjithpaul
#  Unauthorized removal of this header is prohibited.
# ================================================================
"""
API Client for communicating with the Rust backend API
"""
import requests
import os
from typing import Optional, Dict, List, Any

# Get BASE_URL from environment variable or use default
# BASE_URL is for server-to-server communication (Docker internal)
BASE_URL = os.getenv("BASE_URL", "http://localhost:8000")

# PUBLIC_API_URL is for browser-accessible links (must be localhost or public IP)
PUBLIC_API_URL = os.getenv("PUBLIC_API_URL", "http://localhost:8000")

def login(email: str, password: str) -> Optional[str]:
    """
    Login and get JWT token
    
    Args:
        email: Admin email
        password: Admin password
        
    Returns:
        JWT token if successful, None otherwise
    """
    try:
        response = requests.post(
            f"{BASE_URL}/auth/login",
            json={"email": email, "password": password}
        )
        if response.status_code == 200:
            data = response.json()
            return data.get("token")
        return None
    except Exception as e:
        print(f"Login error: {e}")
        return None

def register(email: str, password: str) -> Optional[str]:
    """
    Register a new admin and get JWT token
    
    Args:
        email: Admin email
        password: Admin password
        
    Returns:
        JWT token if successful, None otherwise
    """
    try:
        response = requests.post(
            f"{BASE_URL}/auth/register",
            json={"email": email, "password": password}
        )
        if response.status_code == 200:
            data = response.json()
            return data.get("token")
        return None
    except Exception as e:
        print(f"Registration error: {e}")
        return None

def get_headers(token: str) -> Dict[str, str]:
    """Get headers with authorization token"""
    return {"Authorization": f"Bearer {token}"}

def get_employees(token: str) -> List[Dict[str, Any]]:
    """Get all employees"""
    try:
        response = requests.get(
            f"{BASE_URL}/employees",
            headers=get_headers(token)
        )
        if response.status_code == 200:
            data = response.json()
            if data.get("status") == "success" and data.get("data"):
                return data["data"]
        return []
    except Exception as e:
        print(f"Error fetching employees: {e}")
        return []

def get_employee(token: str, emp_id: int) -> Optional[Dict[str, Any]]:
    """Get employee by ID"""
    try:
        response = requests.get(
            f"{BASE_URL}/employees/{emp_id}",
            headers=get_headers(token)
        )
        if response.status_code == 200:
            data = response.json()
            if data.get("status") == "success" and data.get("data"):
                return data["data"]
        return None
    except Exception as e:
        print(f"Error fetching employee: {e}")
        return None

def create_employee(token: str, employee_data: Dict[str, Any]) -> Optional[Dict[str, Any]]:
    """Create a new employee"""
    try:
        response = requests.post(
            f"{BASE_URL}/employees",
            json=employee_data,
            headers=get_headers(token)
        )
        if response.status_code == 200:
            data = response.json()
            if data.get("status") == "success":
                return data.get("data")
        return None
    except Exception as e:
        print(f"Error creating employee: {e}")
        return None

def update_employee(token: str, emp_id: int, employee_data: Dict[str, Any]) -> bool:
    """Update an employee"""
    try:
        response = requests.put(
            f"{BASE_URL}/employees/{emp_id}",
            json=employee_data,
            headers=get_headers(token)
        )
        return response.status_code == 200
    except Exception as e:
        print(f"Error updating employee: {e}")
        return False

def delete_employee(token: str, emp_id: int) -> bool:
    """Delete an employee"""
    try:
        response = requests.delete(
            f"{BASE_URL}/employees/{emp_id}",
            headers=get_headers(token)
        )
        return response.status_code == 200
    except Exception as e:
        print(f"Error deleting employee: {e}")
        return False

def get_employment_history(token: str, emp_id: int) -> List[Dict[str, Any]]:
    """Get employment history for an employee"""
    try:
        response = requests.get(
            f"{BASE_URL}/employees/{emp_id}/history",
            headers=get_headers(token)
        )
        if response.status_code == 200:
            data = response.json()
            if data.get("status") == "success" and data.get("data"):
                return data["data"]
        return []
    except Exception as e:
        print(f"Error fetching employment history: {e}")
        return []

def add_employment_history(token: str, emp_id: int, history_data: Dict[str, Any]) -> bool:
    """Add employment history for an employee"""
    try:
        response = requests.post(
            f"{BASE_URL}/employees/{emp_id}/history",
            json=history_data,
            headers=get_headers(token)
        )
        return response.status_code == 200
    except Exception as e:
        print(f"Error adding employment history: {e}")
        return False

def upload_payslip(token: str, emp_id: int, file_path: str) -> Optional[Dict[str, Any]]:
    """Upload a payslip PDF for an employee"""
    try:
        with open(file_path, 'rb') as f:
            files = {'file': (file_path.split('/')[-1], f, 'application/pdf')}
            response = requests.post(
                f"{BASE_URL}/employees/{emp_id}/payslip",
                files=files,
                headers=get_headers(token)
            )
            if response.status_code == 200:
                data = response.json()
                if data.get("status") == "success":
                    return data.get("data")
        return None
    except Exception as e:
        print(f"Error uploading payslip: {e}")
        return None

def list_payslips(token: str, emp_id: int) -> List[Dict[str, Any]]:
    """List all payslips for an employee"""
    try:
        response = requests.get(
            f"{BASE_URL}/employees/{emp_id}/payslips",
            headers=get_headers(token)
        )
        if response.status_code == 200:
            data = response.json()
            if data.get("status") == "success" and data.get("data"):
                return data["data"]
        return []
    except Exception as e:
        print(f"Error listing payslips: {e}")
        return []

def get_payslip_url(filename: str) -> str:
    """Get URL for downloading a payslip (browser-accessible)"""
    return f"{PUBLIC_API_URL}/payslip/{filename}"

def get_audit_logs(token: str) -> List[Dict[str, Any]]:
    """Get audit logs"""
    try:
        response = requests.get(
            f"{BASE_URL}/audit_logs",
            headers=get_headers(token)
        )
        if response.status_code == 200:
            data = response.json()
            if data.get("status") == "success" and data.get("data"):
                return data["data"]
        return []
    except Exception as e:
        print(f"Error fetching audit logs: {e}")
        return []

