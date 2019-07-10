<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Devide Calculator</name>
   <tag></tag>
   <elementGuidId>e09d7469-9513-44f4-bab8-892b63930f00</elementGuidId>
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
    &lt;Divide xmlns=&quot;http://CalculatorService&quot;>
      &lt;n1>${n1}&lt;/n1>
      &lt;n2>${n2}&lt;/n2>
    &lt;/Divide>
  &lt;/s:Body>
&lt;/s:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>Divide</soapServiceFunction>
   <variables>
      <defaultValue>12</defaultValue>
      <description></description>
      <id>75ee4cad-1905-4347-b6dd-b43ebb85e8ff</id>
      <masked>false</masked>
      <name>n1</name>
   </variables>
   <variables>
      <defaultValue>2</defaultValue>
      <description></description>
      <id>48d0a0e8-3b5d-46da-8101-a3209c3943d0</id>
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

WS.verifyElementText(response, 'DivideResponse.DivideResult', '6')</verificationScript>
   <wsdlAddress>http://webservice.toscacloud.com/Soap11.svc?wsdl</wsdlAddress>
</WebServiceRequestEntity>
