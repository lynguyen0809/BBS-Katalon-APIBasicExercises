<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Add Calculator</name>
   <tag></tag>
   <elementGuidId>06226d4e-7710-4405-b096-b8c3b6a9f843</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;?xml version=&quot;1.0&quot; encoding=&quot;utf-8&quot;?>
&lt;s:Envelope xmlns:s=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot;>
  &lt;s:Body>
    &lt;Add xmlns=&quot;http://CalculatorService&quot;>
      &lt;n1>${n1}&lt;/n1>
      &lt;n2>${n2}&lt;/n2>
    &lt;/Add>
  &lt;/s:Body>
&lt;/s:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>Add</soapServiceFunction>
   <variables>
      <defaultValue>12</defaultValue>
      <description></description>
      <id>662c2dae-40ad-4da4-80f9-deec8049ecb7</id>
      <masked>false</masked>
      <name>n1</name>
   </variables>
   <variables>
      <defaultValue>2</defaultValue>
      <description></description>
      <id>480bf00e-34b9-4d6d-bc4e-ead43d584488</id>
      <masked>false</masked>
      <name>n2</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyResponseStatusCode(response.getStatusCode(), 200)

WS.verifyElementText(response, 'AddResponse.AddResult', '14')</verificationScript>
   <wsdlAddress>http://webservice.toscacloud.com/Soap11.svc?wsdl</wsdlAddress>
</WebServiceRequestEntity>
