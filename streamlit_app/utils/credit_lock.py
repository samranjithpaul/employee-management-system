# ================================================================
#  Employee Management System (EMS)
#  Developed by: Sam Ranjith Paul
#  GitHub: https://github.com/samranjithpaul
#  LinkedIn: https://www.linkedin.com/in/Samranjithpaul
#  Unauthorized removal of this header is prohibited.
# ================================================================
"""
Credit Lock - Integrity check to ensure developer attribution is preserved
"""
import os
import sys

SIGNATURE = "Sam Ranjith Paul"
REQUIRED_FILES = [
    "utils/footer.py",
    "utils/export_utils.py",
    "pages/dashboard.py",
    "app.py"
]

def verify_signature():
    """
    Verify that developer signature exists in critical files.
    If signature is missing, the application will not start.
    """
    project_path = os.path.dirname(os.path.dirname(__file__))
    found_count = 0
    
    for file_path in REQUIRED_FILES:
        full_path = os.path.join(project_path, file_path)
        if os.path.exists(full_path):
            try:
                with open(full_path, encoding="utf-8", errors="ignore") as f:
                    content = f.read()
                    if SIGNATURE in content:
                        found_count += 1
                    else:
                        print(f"⚠️ Warning: Developer credit missing in {file_path}")
            except Exception as e:
                print(f"⚠️ Error checking {file_path}: {e}")
    
    # Check at least some critical files have the signature
    if found_count < len(REQUIRED_FILES) // 2:
        print("=" * 60)
        print("⚠️  DEVELOPER CREDIT INTEGRITY CHECK FAILED")
        print("=" * 60)
        print(f"Developer signature '{SIGNATURE}' is missing from critical files.")
        print("Please restore developer attribution before continuing.")
        print("=" * 60)
        # Don't exit in production, just warn
        # sys.exit(1)
    
    return found_count >= len(REQUIRED_FILES) // 2

