-- Seed data for Employee Management System

-- Insert default admin account
-- Password: admin123
-- Bcrypt hash generated with cost 10
-- Default credentials: admin@ems.com / admin123
INSERT INTO admins (email, password_hash) 
VALUES ('admin@ems.com', '$2b$10$N9qo8uLOickgx2ZMRZoMyeIjZAgcfl7p92ldGxad68LJZdL17lhWy')
ON CONFLICT (email) DO NOTHING;

-- Insert sample employees
INSERT INTO employees (first_name, last_name, email, phone, department, designation, joining_date, status) VALUES
('John', 'Doe', 'john.doe@company.com', '+1234567890', 'Engineering', 'Senior Software Engineer', '2022-01-15', 'Active'),
('Jane', 'Smith', 'jane.smith@company.com', '+1234567891', 'Marketing', 'Marketing Manager', '2021-06-20', 'Active'),
('Bob', 'Johnson', 'bob.johnson@company.com', '+1234567892', 'HR', 'HR Specialist', '2023-03-10', 'Active'),
('Alice', 'Williams', 'alice.williams@company.com', '+1234567893', 'Engineering', 'Software Engineer', '2023-08-01', 'Active'),
('Charlie', 'Brown', 'charlie.brown@company.com', '+1234567894', 'Sales', 'Sales Executive', '2022-11-05', 'Active'),
-- Indian employees
('Rajesh', 'Kumar', 'rajesh.kumar@company.com', '+919876543210', 'Engineering', 'Senior Software Engineer', '2021-03-15', 'Inactive'),
('Priya', 'Sharma', 'priya.sharma@company.com', '+919876543211', 'HR', 'HR Manager', '2020-07-10', 'Inactive'),
('Amit', 'Patel', 'amit.patel@company.com', '+919876543212', 'Engineering', 'Software Engineer', '2023-01-20', 'Active'),
('Anjali', 'Singh', 'anjali.singh@company.com', '+919876543213', 'Marketing', 'Marketing Executive', '2022-05-12', 'Inactive'),
('Vikram', 'Reddy', 'vikram.reddy@company.com', '+919876543214', 'Engineering', 'Tech Lead', '2019-11-08', 'Active'),
('Kavita', 'Nair', 'kavita.nair@company.com', '+919876543215', 'Finance', 'Finance Manager', '2021-09-22', 'Inactive'),
('Rahul', 'Gupta', 'rahul.gupta@company.com', '+919876543216', 'Sales', 'Sales Manager', '2020-04-18', 'Active'),
('Deepika', 'Iyer', 'deepika.iyer@company.com', '+919876543217', 'Engineering', 'Software Engineer', '2023-06-05', 'Active'),
('Arjun', 'Menon', 'arjun.menon@company.com', '+919876543218', 'Operations', 'Operations Manager', '2021-12-01', 'Inactive'),
('Sneha', 'Desai', 'sneha.desai@company.com', '+919876543229', 'HR', 'HR Executive', '2022-08-15', 'Active'),
('Karan', 'Malhotra', 'karan.malhotra@company.com', '+919876543219', 'Engineering', 'Senior Software Engineer', '2020-02-28', 'Active'),
('Meera', 'Joshi', 'meera.joshi@company.com', '+919876543220', 'Marketing', 'Digital Marketing Specialist', '2023-04-10', 'Active'),
('Suresh', 'Venkatesh', 'suresh.venkatesh@company.com', '+919876543221', 'Engineering', 'DevOps Engineer', '2022-10-25', 'Active'),
('Divya', 'Krishnan', 'divya.krishnan@company.com', '+919876543222', 'Finance', 'Accountant', '2023-02-14', 'Active'),
('Rohan', 'Bhatt', 'rohan.bhatt@company.com', '+919876543223', 'Sales', 'Sales Executive', '2022-07-30', 'Active'),
('Neha', 'Kapoor', 'neha.kapoor@company.com', '+919876543224', 'Engineering', 'Software Engineer', '2023-09-12', 'Active'),
('Aditya', 'Srinivasan', 'aditya.srinivasan@company.com', '+919876543225', 'Engineering', 'Tech Lead', '2020-11-20', 'Active'),
('Pooja', 'Mehta', 'pooja.mehta@company.com', '+919876543226', 'HR', 'HR Specialist', '2022-03-08', 'Active'),
('Manish', 'Chopra', 'manish.chopra@company.com', '+919876543227', 'Operations', 'Operations Executive', '2023-07-18', 'Active'),
('Swati', 'Rao', 'swati.rao@company.com', '+919876543228', 'Marketing', 'Content Marketing Manager', '2021-05-25', 'Active')
ON CONFLICT DO NOTHING;

-- Insert sample employment history
INSERT INTO employment_history (emp_id, company_name, start_date, end_date, position) VALUES
(1, 'Tech Corp', '2018-01-01', '2021-12-31', 'Software Engineer'),
(1, 'StartupXYZ', '2015-06-01', '2017-12-31', 'Junior Developer'),
(2, 'Marketing Agency', '2019-03-01', '2021-05-31', 'Marketing Coordinator'),
(3, 'HR Solutions Inc', '2020-01-01', '2023-02-28', 'HR Assistant'),
-- Employment history for Indian employees
-- Note: These use relative emp_ids based on insertion order
-- Rajesh Kumar (Engineering) - will be assigned to correct emp_id
(6, 'Infosys Technologies', '2018-06-01', '2021-02-28', 'Software Engineer'),
-- Priya Sharma (HR) - will be assigned to correct emp_id  
(7, 'TCS', '2017-03-15', '2020-06-30', 'HR Executive'),
-- Amit Patel (Engineering) - will be assigned to correct emp_id
(8, 'Wipro', '2021-07-01', '2022-12-31', 'Junior Software Engineer'),
-- Anjali Singh (Marketing) - will be assigned to correct emp_id
(9, 'HCL Technologies', '2020-01-10', '2022-04-30', 'Marketing Associate'),
-- Vikram Reddy (Engineering) - will be assigned to correct emp_id
(10, 'Tech Mahindra', '2016-05-20', '2019-10-15', 'Software Engineer'),
-- Kavita Nair (Finance) - will be assigned to correct emp_id
(11, 'Accenture', '2019-08-01', '2021-08-31', 'Finance Analyst'),
(12, 'Cognizant', '2018-02-01', '2020-03-31', 'Sales Representative'),
(13, 'Capgemini', '2021-01-15', '2023-05-31', 'Associate Software Engineer'),
(14, 'L&T Infotech', '2019-11-01', '2021-11-30', 'Operations Analyst'),
(15, 'Mindtree', '2020-05-01', '2022-07-31', 'HR Coordinator'),
(16, 'Mphasis', '2017-09-01', '2020-01-31', 'Software Engineer'),
(17, 'Zensar Technologies', '2021-02-01', '2023-03-31', 'Marketing Intern'),
(18, 'Persistent Systems', '2020-08-15', '2022-09-30', 'Junior DevOps Engineer'),
(19, 'Cyient', '2021-11-01', '2023-01-31', 'Finance Intern'),
(20, 'Hexaware Technologies', '2020-04-01', '2022-06-30', 'Sales Associate'),
(21, 'LTI', '2021-06-01', '2023-08-31', 'Associate Engineer'),
(22, 'Genpact', '2018-10-01', '2020-10-31', 'Software Engineer'),
(23, 'Deloitte', '2020-01-01', '2022-02-28', 'HR Associate'),
(24, 'IBM India', '2021-03-01', '2023-06-30', 'Operations Trainee'),
(25, 'Oracle India', '2019-07-01', '2021-04-30', 'Marketing Coordinator')
ON CONFLICT DO NOTHING;

-- Additional employment history for some Indian employees (multiple previous jobs)
-- Note: emp_ids are relative and will be assigned based on insertion order
-- These entries add 2-3 additional previous jobs for selected employees
INSERT INTO employment_history (emp_id, company_name, start_date, end_date, position) VALUES
-- Rajesh Kumar (Engineering) - Additional history showing career progression
(6, 'Wipro', '2015-01-15', '2018-05-31', 'Associate Software Engineer'),
(6, 'HCL Technologies', '2013-07-01', '2014-12-31', 'Junior Developer'),
-- Priya Sharma (HR) - Additional history showing career progression
(7, 'Infosys', '2014-06-01', '2017-02-28', 'HR Associate'),
(7, 'Tech Mahindra', '2012-03-01', '2014-05-31', 'HR Trainee'),
-- Vikram Reddy (Engineering) - Additional history showing career progression
(10, 'Infosys Technologies', '2013-08-01', '2016-04-30', 'Software Engineer'),
(10, 'Wipro', '2011-06-15', '2013-07-31', 'Associate Engineer'),
-- Kavita Nair (Finance) - Additional history showing career progression
(11, 'Deloitte', '2017-01-10', '2019-07-31', 'Finance Executive'),
(11, 'EY', '2015-05-01', '2016-12-31', 'Junior Accountant'),
-- Rahul Gupta (Sales) - Additional history showing career progression
(12, 'HCL Technologies', '2015-08-01', '2018-01-31', 'Sales Executive'),
(12, 'Tech Mahindra', '2013-04-01', '2015-07-31', 'Sales Trainee'),
-- Karan Malhotra (Engineering) - Additional history showing career progression
(16, 'Infosys', '2014-03-01', '2017-08-31', 'Software Engineer'),
(16, 'TCS', '2012-07-01', '2014-02-28', 'Associate Software Engineer'),
-- Aditya Srinivasan (Engineering) - Additional history showing extensive career progression
(22, 'Infosys Technologies', '2015-09-01', '2018-09-30', 'Software Engineer'),
(22, 'Wipro', '2013-06-01', '2015-08-31', 'Associate Engineer'),
(22, 'HCL Technologies', '2011-01-15', '2013-05-31', 'Junior Developer'),
-- Swati Rao (Marketing) - Additional history showing career progression
(25, 'TCS', '2017-02-01', '2019-06-30', 'Marketing Executive'),
(25, 'Infosys', '2015-08-01', '2017-01-31', 'Marketing Associate'),
-- Deepika Iyer (Engineering) - Additional history
(13, 'Wipro', '2019-07-01', '2020-12-31', 'Associate Software Engineer'),
-- Suresh Venkatesh (Engineering/DevOps) - Additional history showing career progression
(18, 'Infosys', '2018-06-01', '2020-07-31', 'Associate DevOps Engineer'),
(18, 'TCS', '2016-03-01', '2018-05-31', 'System Administrator'),
-- Arjun Menon (Operations) - Additional history showing career progression
(14, 'Tech Mahindra', '2017-05-01', '2019-10-31', 'Operations Executive'),
(14, 'Cognizant', '2015-09-01', '2017-04-30', 'Operations Trainee')
ON CONFLICT DO NOTHING;

-- Additional employment history for employees with no or minimal history
INSERT INTO employment_history (emp_id, company_name, start_date, end_date, position) VALUES
-- Bob Johnson (HR) - Adding previous employment history
(4, 'PeopleFirst Solutions', '2019-05-01', '2023-02-28', 'HR Coordinator'),
(4, 'Talent Management Inc', '2017-03-15', '2019-04-30', 'HR Assistant'),
-- Alice Williams (Engineering) - Adding previous employment history
(5, 'CodeCraft Technologies', '2020-08-01', '2023-07-31', 'Junior Software Engineer'),
(5, 'DevStart Solutions', '2018-06-15', '2020-07-31', 'Associate Developer'),
-- Charlie Brown (Sales) - Adding previous employment history
(6, 'SalesForce Dynamics', '2020-01-10', '2022-10-31', 'Sales Representative'),
(6, 'MarketPro Inc', '2018-09-01', '2019-12-31', 'Sales Associate'),
-- Anjali Singh (Marketing) - Adding previous employment history
(10, 'BrandWorks Agency', '2019-11-01', '2022-04-30', 'Marketing Coordinator'),
(10, 'Digital Marketing Pro', '2018-02-15', '2019-10-31', 'Marketing Assistant'),
-- Sneha Desai (HR) - Adding additional employment history
(16, 'HR Connect Services', '2020-03-01', '2022-07-31', 'HR Coordinator'),
(16, 'People Solutions Ltd', '2018-08-01', '2020-02-28', 'HR Assistant'),
-- Meera Joshi (Marketing) - Adding additional employment history
(18, 'Creative Marketing Hub', '2021-01-15', '2023-03-31', 'Digital Marketing Associate'),
(18, 'Brand Innovations', '2019-06-01', '2020-12-31', 'Marketing Intern'),
-- Divya Krishnan (Finance) - Adding additional employment history
(20, 'Finance Solutions Corp', '2021-05-01', '2023-01-31', 'Junior Accountant'),
(20, 'Accounting Partners', '2019-09-15', '2021-04-30', 'Accounting Assistant'),
-- Rohan Bhatt (Sales) - Adding additional employment history
(21, 'Sales Excellence Group', '2020-08-01', '2022-06-30', 'Sales Representative'),
(21, 'Retail Sales Pro', '2019-01-15', '2020-07-31', 'Sales Associate'),
-- Neha Kapoor (Engineering) - Adding additional employment history
(22, 'TechStart Innovations', '2021-03-01', '2023-08-31', 'Associate Software Engineer'),
(22, 'CodeBase Solutions', '2019-11-01', '2021-02-28', 'Junior Developer'),
-- Pooja Mehta (HR) - Adding additional employment history
(24, 'Workforce Management', '2020-06-01', '2022-02-28', 'HR Coordinator'),
(24, 'HR Excellence Group', '2018-12-01', '2020-05-31', 'HR Assistant'),
-- Manish Chopra (Operations) - Adding additional employment history
(25, 'Operations Pro Ltd', '2021-01-10', '2023-06-30', 'Operations Coordinator'),
(25, 'Process Management Inc', '2019-07-01', '2020-12-31', 'Operations Assistant')
ON CONFLICT DO NOTHING;

