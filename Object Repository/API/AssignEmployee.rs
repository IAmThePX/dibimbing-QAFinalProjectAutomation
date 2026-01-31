<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>AssignEmployee</name>
   <tag></tag>
   <elementGuidId>05ec4b1b-8dda-4098-8ba0-0430e5336b5b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;query\&quot;: \&quot;mutation AssignProgram($input: AssignProgramInput!) { assignProgram(input: $input) }\&quot;,\n  \&quot;variables\&quot;: \&quot;{ \\\&quot;input\\\&quot;: { \\\&quot;programId\\\&quot;: \\\&quot;f83e75a5-d2ac-43e7-92be-12fc109a5ff6\\\&quot;, \\\&quot;employeeIds\\\&quot;: [ \\\&quot;fe2d8829-4432-47c1-aa17-38dd22279989\\\&quot; ], \\\&quot;startDate\\\&quot;: \\\&quot;2026-08-30T00:00:00.000Z\\\&quot;, \\\&quot;endDate\\\&quot;: \\\&quot;2027-12-30T00:00:00.000Z\\\&quot; } }\&quot;\n}&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;displayText&quot;: &quot;mutation AssignProgram($input: AssignProgramInput!) {\n  assignProgram(input: $input)\n}\n&quot;,
  &quot;displayVariables&quot;: &quot;{\n  \&quot;input\&quot;: {\n    \&quot;programId\&quot;: \&quot;f83e75a5-d2ac-43e7-92be-12fc109a5ff6\&quot;,\n    \&quot;employeeIds\&quot;: [\n      \&quot;fe2d8829-4432-47c1-aa17-38dd22279989\&quot;\n    ],\n    \&quot;startDate\&quot;: \&quot;2026-08-30T00:00:00.000Z\&quot;,\n    \&quot;endDate\&quot;: \&quot;2027-12-30T00:00:00.000Z\&quot;\n  }\n}\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;
}</httpBodyContent>
   <httpBodyType>GraphQL</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>93ed229f-d255-4a00-a22f-14372bd130f8</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Cookie</name>
      <type>Main</type>
      <value>sid_b2b=s%3AyC-N4RnpLGdLH2CrcktqpgOZB6ETrS7P.txuxnTQQFvdlDPzfi%2BGT4mweQXDIabcMyVDLmrrKH5A; __cf_bm=_HdloFO9toPL5jfjbjYUUp3_3.W_07aeBxTQzbgFDts-1769832188-1.0.1.1-cuE8FRwbNsNWFMmn.hM_y13itbkU.WULHPqk9hdABcHAm6I3_X7d4ZWBYUeLd9czuOnfgLSWVWCR9Lz9q9Vt7TZJqsXGfjWY.2hWmCHAg.c</value>
      <webElementGuid>70b72488-3b03-4150-982b-fd74e0adffd2</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.4.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://lmsb2b.do.dibimbing.id/graphql</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
