# LMS B2B QA Automation (UI & API)

Automation testing project for **LMS B2B Dibimbing**, covering:

- ✅ UI Automation (Web)
- ✅ API Automation (GraphQL)
- ✅ End-to-End Flow (Employee → Program → Assignment)
- ✅ CI Integration using GitHub Actions & Katalon Studio Engine (Free Edition)

---

## 🧪 Tech Stack

- **Katalon Studio (Free Edition)**  
  > Note: CI runs using CLI engine, Free Edition does not require KRE.  
- **Katalon Studio Engine (CLI)**  
- **Groovy**  
- **GraphQL**  
- **GitHub Actions**  
- **Browsers**:
  - Firefox (recommended for CI, headless)
  - Chrome / Edge for local testing

---

## 📂 Project Structure

.
├── Test Cases
│ ├── API
│ │ └── E2E_API_Employee_Training
│ └── UI
│ └── E2E_UI_Employee_Training
├── Test Suites
│ └── TS_E2E_All
├── Object Repository
│ ├── API
│ └── UI
├── Reports
├── README.md
└── *.prj


---

## 🔁 End-to-End Test Flow

### 🔹 API Flow (GraphQL)
1. Login
2. Create Employee (dynamic email)
3. Create Program (Training)
4. Create Chapter
5. Create Content
6. Assign Program to Employee
7. Verify assignment
8. Delete Employee (cleanup)

### 🔹 UI Flow (Web)
1. Login
2. Add Employee
3. Add Division
4. Create Training Program
5. Add Chapter & Content
6. Assign Employee to Training
7. Verify assignment on Employee detail

---

## ⚠️ Known Issues (IMPORTANT)

### 🧊 Soft Freeze on Chrome & Edge
Observed **soft-freeze / UI hang** on:
- Opening **Assign Employee modal**
- Searching employee in Assign flow
- Occasionally during Login page load

✔️ **Does NOT occur in Firefox (headless recommended for CI)**

📌 **Root Cause (suspected)**:
- Frontend overload / heavy JS rendering
- Browser-specific behavior (Chrome / Edge)

👉 **Recommendation**:
- Use **Firefox (headless)** for CI execution
- Chrome & Edge are still valid for local testing

---

## 🚀 Running Tests Locally

### UI Test

# Run UI tests locally
$KATALON_BIN \
  -noSplash \
  -runMode=console \
  -projectPath="LMS_B2B_QA_Automation-LK.prj" \
  -testSuitePath="Test Suites/TS_E2E_All.ts" \
  -browserType=Firefox \
  -executionProfile=default \
  --config -webui.autoUpdateDrivers=true
