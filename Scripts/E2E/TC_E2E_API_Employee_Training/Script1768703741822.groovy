import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import java.time.ZonedDateTime
import java.time.format.DateTimeFormatter

// ======================
// LOGIN
// ======================
def loginRes = WS.sendRequest(findTestObject('API/Login'))

WS.verifyResponseStatusCode(loginRes, 200)

// ======================
// DYNAMIC DATA
// ======================
def ts = System.currentTimeMillis()

long startDate = System.currentTimeMillis()

long endDate   = startDate + (5 * 24 * 60 * 60 * 1000) // +5 hari

def email = "dummy_qa_$ts@mail.com"

def name = "Dummy QA $ts"

String employeeId

String programId

// ======================
// CREATE EMPLOYEE
// ======================
def empRes = WS.sendRequest(
    findTestObject(
        'API/CreateEmployee',
        [
            ('name')         : name,
            ('email')        : email,
            ('username')     : email,
            ('employeeRole') : 'QA Tester',
            ('phoneNumber')  : '08123456789'
        ]
    )
)

WS.verifyResponseStatusCode(empRes, 200)

employeeId = WS.getElementPropertyValue(empRes, 'data.createEmployee.id')

assert employeeId != null

// ======================
// CREATE PROGRAM
// ======================
def progRes = WS.sendRequest(
    findTestObject(
        'API/CreateProgram',
        [
            ('type')  : 'training',
            ('title') : "Automation Training $ts"
        ]
    )
)

WS.verifyResponseStatusCode(progRes, 200)

programId = WS.getElementPropertyValue(progRes, 'data.createProgram.id')

assert programId != null

// ======================
// CREATE CHAPTER
// ======================
def chapterRes = WS.sendRequest(findTestObject('API/CreateChapter', [('programId') : programId, ('title') : 'Intro Chapter'
            , ('order') : 1]))

WS.verifyResponseStatusCode(chapterRes, 200)

// ❗ Tidak assert response body (backend tidak konsisten)
// ======================
// CREATE CONTENT
// ======================
def contentRes = WS.sendRequest(findTestObject('API/CreateContent', [('programId') : programId, ('title') : 'Intro Video'
            , ('type') : 'video', ('duration') : 1, ('order') : 1]))

WS.verifyResponseStatusCode(contentRes, 200)

// ======================
// ASSIGN PROGRAM (FINAL GOAL)
// ======================
def assignRes = WS.sendRequest(
    findTestObject('API/AssignEmployee', [
        ('programId')  : programId,
        ('employeeId') : employeeId,
        ('startDate')  : startDate,
        ('endDate')    : endDate
    ])
)

WS.verifyResponseStatusCode(assignRes, 200)

def assigned = WS.getElementPropertyValue(assignRes, 'data.assignProgram')

assert assigned == true

// ======================
// DELETE EMPLOYEE
// ======================
def deleteRes = WS.sendRequest(findTestObject('API/DeleteEmployee', [('employeeId') : employeeId]))

WS.verifyResponseStatusCode(deleteRes, 200)

