# LMS B2B QA Automation (UI & API)

Automated End-to-End testing project for LMS B2B platform using **Katalon Studio**.
This project covers **UI Automation** and **API Automation (GraphQL)** in a single CI pipeline.

---

## 🔧 Tech Stack

- **Katalon Studio 9.4.0**
- **Groovy**
- **GraphQL API**
- **GitHub Actions (CI)**
- **Browsers Tested**
  - ✅ Firefox (stable)
  - ⚠️ Chrome & Edge (soft freeze issue)

---

## 🧪 Test Coverage

### 1️⃣ UI Automation
End-to-End scenario:
- Login
- Create Employee
- Create Division
- Create Training Program
- Add Chapter & Content
- Assign Employee to Training
- Verify Assignment on Employee Detail

📌 **Known Issue (Documented):**
- Soft freeze occurs on **Chrome & Edge** when:
  - Opening *Assign Employee modal*
  - Searching employee inside modal
  - Occasionally on login page (high load)
- **Firefox does NOT experience soft freeze**, only longer loading time.
- Therefore, **Firefox is used as the default browser for CI**.

---

### 2️⃣ API Automation (GraphQL)

End-to-End API flow:
- Login
- Create Employee (dynamic unique data)
- Create Program
- Create Chapter
- Create Content
- Assign Program to Employee
- Delete Employee (cleanup)

📌 Important Notes:
- GraphQL `DateTime` **must be ISO 8601 string**
- API errors previously encountered were **backend input validation issues**, not test case issues
- Guard assertions are applied for `data != null` to prevent false-negative failures

---

## ▶️ How to Run Locally

### UI Test
```bash
katalonc \
  -runMode=console \
  -projectPath="LMS_B2B_QA_Automation-LK.prj" \
  -testSuitePath="Test Suites/TS_E2E_All" \
  -executionProfile="default" \
  -browserType="Firefox"
