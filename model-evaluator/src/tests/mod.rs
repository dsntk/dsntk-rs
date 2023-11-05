use crate::model_evaluator::ModelEvaluator;
use dsntk_feel::context::FeelContext;
use dsntk_feel::values::Value;
use dsntk_feel::FeelScope;
use once_cell::sync::Lazy;
use std::sync::Arc;

mod compatibility;
mod various;

macro_rules! from_examples {
  ($model_name:tt) => {
    model_evaluator_from_examples!($model_name);
    model_namespace_from_examples!($model_name);
    model_name_from_examples!($model_name);
  };
}

macro_rules! model_evaluator_from_examples {
  ($model_name:tt) => {
    static MODEL_EVALUATOR: Lazy<Arc<ModelEvaluator>> = Lazy::new(|| build_model_evaluator(dsntk_examples::$model_name));
  };
}

macro_rules! model_namespace_from_examples {
  ($model_name:tt) => {
    static MODEL_NAMESPACE: Lazy<String> = Lazy::new(|| build_model_namespace(dsntk_examples::$model_name));
  };
}

macro_rules! model_name_from_examples {
  ($model_name:tt) => {
    static MODEL_NAME: Lazy<String> = Lazy::new(|| build_model_name(dsntk_examples::$model_name));
  };
}

macro_rules! model_evaluator {
  ($model_name:tt) => {
    static MODEL_EVALUATOR: Lazy<Arc<ModelEvaluator>> = Lazy::new(|| build_model_evaluator($model_name));
  };
}

macro_rules! static_context {
  ($name:tt, $content:tt) => {
    static $name: Lazy<FeelContext> = Lazy::new(|| context($content));
  };
}

use dsntk_feel_evaluator::BuildContext;
use dsntk_model::NamedElement;
use {from_examples, model_evaluator, model_evaluator_from_examples, model_name_from_examples, model_namespace_from_examples, static_context};

/// Utility function that creates a `FEEL` context from specified input expression.
pub fn context(input: &str) -> FeelContext {
  let scope = FeelScope::default();
  match dsntk_feel_parser::parse_context(&scope, input, false) {
    Ok(node) => {
      let evaluator = dsntk_feel_evaluator::prepare(&BuildContext::default(), &node);
      match evaluator(&scope) {
        Value::Context(ctx) => ctx,
        other => panic!("ERROR: expected context value, actual value is: {}", other as Value),
      }
    }
    Err(reason) => panic!("ERROR: parsing context failed with reason: {reason}"),
  }
}

/// Utility function that builds a model evaluator from single XML model definitions.
fn build_model_evaluator(model_content: &str) -> Arc<ModelEvaluator> {
  let definitions = dsntk_model::parse(model_content).unwrap();
  ModelEvaluator::new(&[definitions]).unwrap()
}

/// Utility function that builds a model evaluator from multiple XML model definitions.
fn build_model_evaluators(model_content: &[&str]) -> Arc<ModelEvaluator> {
  let mut definitions = vec![];
  for content in model_content {
    definitions.push(dsntk_model::parse(content).unwrap());
  }
  ModelEvaluator::new(&definitions).unwrap()
}

/// Utility function that returns a model namespace from a single DMN model.
fn build_model_namespace(model_content: &str) -> String {
  let definitions = dsntk_model::parse(model_content).unwrap();
  definitions.namespace().to_string()
}

/// Utility function that returns a model names from a single DMN model.
fn build_model_name(model_content: &str) -> String {
  let definitions = dsntk_model::parse(model_content).unwrap();
  definitions.name().to_string()
}

/// Utility function that evaluates a [Decision] specified by name and compares the result.
fn assert_decision(model_evaluator: &ModelEvaluator, model_namespace: &str, model_name: &str, invocable_name: &str, input_data: &FeelContext, expected: &str) {
  let actual = model_evaluator.evaluate_invocable(model_namespace, model_name, invocable_name, input_data).to_string();
  assert_eq!(
    expected, actual,
    "Assertion error, actual value of the decision does not match the expected value:\n  expected: {expected}\n    actual: {actual}\n"
  );
}

/// Utility function that evaluates a [BusinessKnowledgeModel] specified by name and compares the result.
fn assert_business_knowledge_model(model_evaluator: &ModelEvaluator, model_namespace: &str, model_name: &str, invocable_name: &str, input_data: &FeelContext, expected: &str) {
  let actual = model_evaluator.evaluate_invocable(model_namespace, model_name, invocable_name, input_data).to_string();
  assert_eq!(
    expected, actual,
    "Assertion error, actual value of the business knowledge model does not match the expected value:\n  expected: {expected}\n    actual: {actual}\n"
  );
}

/// Utility function that evaluates a [DecisionService] specified by name and compares the result with expected value.
fn assert_decision_service(model_evaluator: &ModelEvaluator, model_namespace: &str, model_name: &str, invocable_name: &str, input: &str, expected: &str) {
  let input_data = context(input);
  let actual = model_evaluator.evaluate_invocable(model_namespace, model_name, invocable_name, &input_data).to_string();
  assert_eq!(
    expected, actual,
    "Assertion error, actual value of the decision service does not match the expected value:\n  expected: {expected}\n    actual: {actual}\n"
  );
}
