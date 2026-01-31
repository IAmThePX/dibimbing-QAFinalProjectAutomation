<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>CreateEmployee</name>
   <tag></tag>
   <elementGuidId>204977cd-71f7-4282-821b-abb6f0311394</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;query\&quot;: \&quot;mutation CreateEmployee($input: EmployeeInput!) { createEmployee(input: $input) { id name email } }\&quot;,\n  \&quot;variables\&quot;: \&quot;{ \\\&quot;input\\\&quot;: { \\\&quot;name\\\&quot;: \\\&quot;${name}\\\&quot;, \\\&quot;email\\\&quot;: \\\&quot;${email}\\\&quot;, \\\&quot;username\\\&quot;: \\\&quot;${username}\\\&quot;, \\\&quot;employeeRole\\\&quot;: \\\&quot;${employeeRole}\\\&quot;, \\\&quot;phoneNumber\\\&quot;: \\\&quot;${phoneNumber}\\\&quot;, \\\&quot;divisionId\\\&quot;: \\\&quot;784d157e-6ea8-4166-8afb-b721fbb0a0e2\\\&quot; } }\&quot;\n}&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;displayText&quot;: &quot;mutation CreateEmployee($input: EmployeeInput!) { createEmployee(input: $input) { id name email } }&quot;,
  &quot;displayVariables&quot;: &quot; {\n    \&quot;input\&quot;: {\n      \&quot;name\&quot;: \&quot;${name}\&quot;,\n      \&quot;email\&quot;: \&quot;${email}\&quot;,\n      \&quot;username\&quot;: \&quot;${username}\&quot;,\n      \&quot;employeeRole\&quot;: \&quot;${employeeRole}\&quot;,\n      \&quot;phoneNumber\&quot;: \&quot;${phoneNumber}\&quot;,\n      \&quot;divisionId\&quot;: \&quot;784d157e-6ea8-4166-8afb-b721fbb0a0e2\&quot;\n    }\n  }\n\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;
}</httpBodyContent>
   <httpBodyType>GraphQL</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>6de6bb2c-499b-4ddf-943c-106dd8140ba8</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Cookie</name>
      <type>Main</type>
      <value>sid_b2b=s%3AyC-N4RnpLGdLH2CrcktqpgOZB6ETrS7P.txuxnTQQFvdlDPzfi%2BGT4mweQXDIabcMyVDLmrrKH5A; __cf_bm=_HdloFO9toPL5jfjbjYUUp3_3.W_07aeBxTQzbgFDts-1769832188-1.0.1.1-cuE8FRwbNsNWFMmn.hM_y13itbkU.WULHPqk9hdABcHAm6I3_X7d4ZWBYUeLd9czuOnfgLSWVWCR9Lz9q9Vt7TZJqsXGfjWY.2hWmCHAg.c</value>
      <webElementGuid>e54ff246-9f8e-4d22-961f-2ac6a9084ef1</webElementGuid>
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
