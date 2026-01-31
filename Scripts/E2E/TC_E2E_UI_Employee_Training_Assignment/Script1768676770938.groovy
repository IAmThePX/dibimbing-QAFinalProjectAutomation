import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.testobject.ConditionType as ConditionType
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint

// ======================
// Open Browser & Login
// ======================
WebUI.openBrowser('')

WebUI.maximizeWindow()

WebUI.navigateToUrl('https://lms-b2b.do.dibimbing.id/')

WebUI.navigateToUrl('https://lms-b2b.do.dibimbing.id/dibimbingqa/login')

WebUI.delay(10)

WebUI.verifyElementPresent(findTestObject('UI/Auth/input_email'), 5)

WebUI.setText(findTestObject('UI/Auth/input_email'), 'arwendymelyn@dibimbing.id')

WebUI.setText(findTestObject('UI/Auth/input_password'), 's2et9bh6l')

WebUI.click(findTestObject('UI/Auth/button_login'))

WebUI.delay(5)

// ======================
// Add Employee
// ======================
WebUI.navigateToUrl('https://lms-b2b.do.dibimbing.id/dibimbingqa/admin/employee?tab=0')

WebUI.waitForPageLoad(5)

String employeeName = 'dummy_' + System.currentTimeMillis()

GlobalVariable.employeeName = employeeName

WebUI.verifyElementPresent(findTestObject('UI/Employee/button-add-employee'), 10)

WebUI.click(findTestObject('UI/Employee/button-add-employee'))

WebUI.setText(findTestObject('UI/Employee/input-name'), employeeName)

WebUI.setText(findTestObject('UI/Employee/input-employeeId'), 'EMP001')

WebUI.setText(findTestObject('UI/Employee/input-email'), employeeName + '@mail.com')

WebUI.setText(findTestObject('UI/Employee/input-phoneNumber'), '08123456789')

WebUI.selectOptionByLabel(findTestObject('UI/Employee/dropdown-division'), 'QA', false)

WebUI.setText(findTestObject('UI/Employee/input-employeeRole'), 'QA Tester')

WebUI.click(findTestObject('UI/Employee/button-add-employee-submit'))

WebUI.navigateToUrl('https://lms-b2b.do.dibimbing.id/dibimbingqa/admin/employee?tab=1')

WebUI.delay(5)

WebUI.click(findTestObject('UI/Division/add-division-button'))

WebUI.setText(findTestObject('UI/Division/input-name'), 'QA Division')

WebUI.setText(findTestObject('UI/Division/input-description'), 'Division Automation')

WebUI.click(findTestObject('UI/Division/add-division-confirm-button'))

WebUI.delay(10)

// ======================
// Create Training
// ======================
WebUI.navigateToUrl('https://lms-b2b.do.dibimbing.id/dibimbingqa/admin/program/training')

WebUI.verifyElementPresent(findTestObject('UI/Training/add-training-button'), 10)

WebUI.click(findTestObject('UI/Training/add-training-button'))

WebUI.setText(findTestObject('UI/Training/input-title'), 'Automation Training')

WebUI.click(findTestObject('UI/Training/add-training-submit-button'))

WebUI.click(findTestObject('UI/Training/button-detail-training-0'))

// ======================
// Add Chapter
// ======================
WebUI.click(findTestObject('UI/Training/add-chapter-button'))

WebUI.setText(findTestObject('UI/Training/input-chapter-title'), 'Intro Chapter')

WebUI.click(findTestObject('UI/Training/add-chapter-submit-button'))

WebUI.click(findTestObject('UI/Training/add-content-button'))

WebUI.setText(findTestObject('UI/Training/input-content-title'), 'Intro Automation Content')

// CKEditor (contenteditable)
TestObject descEditor = new TestObject('ckeditor_desc')

descEditor.addProperty('xpath', ConditionType.EQUALS, '//div[@role=\'textbox\' and @contenteditable=\'true\']')

WebUI.click(descEditor)

WebUI.sendKeys(descEditor, 'Deskripsi konten automation')

WebUI.click(findTestObject('UI/Training/upload-media'))

WebUI.click(findTestObject('UI/Training/video'))

WebUI.click(findTestObject('UI/Training/choose-media-button'))

WebUI.setText(findTestObject('UI/Training/input-content-duration'), '10')

WebUI.click(findTestObject('UI/Training/button-add-content-submit'))

WebUI.verifyTextPresent('Intro Automation Content', false)

WebUI.delay(5)

WebUI.navigateToUrl('https://lms-b2b.do.dibimbing.id/dibimbingqa/admin/program/training/detail/65e7df74-65fe-4ff3-bf55-2862ec3a44ef?tabIndex=1')

WebUI.click(findTestObject('UI/Training/assign-employee-button'))

WebUI.navigateToUrl('https://lms-b2b.do.dibimbing.id/dibimbingqa/admin/program/training/detail/65e7df74-65fe-4ff3-bf55-2862ec3a44ef/assign-employee')

WebUI.delay(5)

WebUI.setText(findTestObject('UI/Training/search-employee'), 'dummy')

WebUI.delay(5)

WebUI.click(findTestObject('UI/Training/button-add-employee'))

WebUI.setText(findTestObject('UI/Training/input-deadline'), '01/02/2026')

WebUI.click(findTestObject('UI/Training/button-assign-employee-submit'))

WebUI.delay(5)

// ======================
// Verify Assignment on Employee Detail
// ======================
WebUI.navigateToUrl('https://lms-b2b.do.dibimbing.id/dibimbingqa/admin/employee?tab=0')

WebUI.setText(findTestObject('UI/Employee/searchbar_employee'), 'dummy')

WebUI.delay(5)

WebUI.navigateToUrl('https://lms-b2b.do.dibimbing.id/dibimbingqa/admin/employee/fe2d8829-4432-47c1-aa17-38dd22279989')

WebUI.delay(2)

WebUI.click(findTestObject('UI/Employee/tab-assigned-program'))

WebUI.verifyTextPresent('Automation', false)

WebUI.delay(5)

WebUI.closeBrowser()

