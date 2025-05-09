Feature: Mortgage Calculator

  Scenario: If default input is submitted then results will be correct
    Given default input data
    When input is submitted
    Then results are correct
