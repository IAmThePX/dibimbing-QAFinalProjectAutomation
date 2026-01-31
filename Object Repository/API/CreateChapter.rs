<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>CreateChapter</name>
   <tag></tag>
   <elementGuidId>03e725d8-34d0-4c04-9b60-133de9d324e6</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;query\&quot;: \&quot;mutation CreateChapter($input: ChapterInput!) { createChapter(input: $input) { id } }\&quot;,\n  \&quot;variables\&quot;: \&quot;{ \\\&quot;input\\\&quot;: { \\\&quot;title\\\&quot;: \\\&quot;Intro Chapter\\\&quot;, \\\&quot;description\\\&quot;: \\\&quot;\\\&quot;, \\\&quot;programId\\\&quot;: \\\&quot;PROGRAM_ID\\\&quot; } }\&quot;\n}&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;displayText&quot;: &quot;mutation CreateChapter($input: ChapterInput!) {\n  createChapter(input: $input) {\n    id\n  }\n}\n&quot;,
  &quot;displayVariables&quot;: &quot;{\n  \&quot;input\&quot;: {\n    \&quot;title\&quot;: \&quot;Intro Chapter\&quot;,\n    \&quot;description\&quot;: \&quot;\&quot;,\n    \&quot;programId\&quot;: \&quot;PROGRAM_ID\&quot;\n  }\n}\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;
}</httpBodyContent>
   <httpBodyType>GraphQL</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>e7689a70-26fb-4262-8382-0367cc3d2d24</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Cookie</name>
      <type>Main</type>
      <value>sid_b2b=s%3AyC-N4RnpLGdLH2CrcktqpgOZB6ETrS7P.txuxnTQQFvdlDPzfi%2BGT4mweQXDIabcMyVDLmrrKH5A; __cf_bm=_HdloFO9toPL5jfjbjYUUp3_3.W_07aeBxTQzbgFDts-1769832188-1.0.1.1-cuE8FRwbNsNWFMmn.hM_y13itbkU.WULHPqk9hdABcHAm6I3_X7d4ZWBYUeLd9czuOnfgLSWVWCR9Lz9q9Vt7TZJqsXGfjWY.2hWmCHAg.c</value>
      <webElementGuid>2044fa3c-e778-4b9d-9acf-c01281381306</webElementGuid>
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
