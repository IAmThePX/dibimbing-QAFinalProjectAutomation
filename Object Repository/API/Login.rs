<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Login</name>
   <tag></tag>
   <elementGuidId>4c7a1a00-84e6-4143-a9dc-395e3deb6506</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>${GlobalVariable.token}</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;query\&quot;: \&quot;mutation Login( $companyId: String! $usernameOrEmail: String! $password: String! ) { login( companyId: $companyId usernameOrEmail: $usernameOrEmail password: $password ) { user { id username role } errors { field message } } }\&quot;,\n  \&quot;variables\&quot;: \&quot;{ \\\&quot;companyId\\\&quot;: \\\&quot;811637b1-9989-4d45-a9f5-220c5f2354f7\\\&quot;, \\\&quot;usernameOrEmail\\\&quot;: \\\&quot;arwendymelyn@dibimbing.id\\\&quot;, \\\&quot;password\\\&quot;: \\\&quot;s2et9bh6l\\\&quot; }\&quot;\n}&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;displayText&quot;: &quot;mutation Login(\n  $companyId: String!\n  $usernameOrEmail: String!\n  $password: String!\n) {\n  login(\n    companyId: $companyId\n    usernameOrEmail: $usernameOrEmail\n    password: $password\n  ) {\n    user {\n      id\n      username\n      role\n    }\n    errors {\n      field\n      message\n    }\n  }\n}\n&quot;,
  &quot;displayVariables&quot;: &quot;{\n  \&quot;companyId\&quot;: \&quot;811637b1-9989-4d45-a9f5-220c5f2354f7\&quot;,\n  \&quot;usernameOrEmail\&quot;: \&quot;arwendymelyn@dibimbing.id\&quot;,\n  \&quot;password\&quot;: \&quot;s2et9bh6l\&quot;\n}\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;
}</httpBodyContent>
   <httpBodyType>GraphQL</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>583a961e-02cb-4fce-ab07-225e66b3f7f2</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Cookie</name>
      <type>Main</type>
      <value>sid_b2b=s%3AyC-N4RnpLGdLH2CrcktqpgOZB6ETrS7P.txuxnTQQFvdlDPzfi%2BGT4mweQXDIabcMyVDLmrrKH5A; __cf_bm=_HdloFO9toPL5jfjbjYUUp3_3.W_07aeBxTQzbgFDts-1769832188-1.0.1.1-cuE8FRwbNsNWFMmn.hM_y13itbkU.WULHPqk9hdABcHAm6I3_X7d4ZWBYUeLd9czuOnfgLSWVWCR9Lz9q9Vt7TZJqsXGfjWY.2hWmCHAg.c</value>
      <webElementGuid>af19f257-1216-4d64-9916-046ae6c6f8b3</webElementGuid>
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
