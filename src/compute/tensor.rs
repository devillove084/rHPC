// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use async_trait::async_trait;
use ndarray::{ArrayD, Dimension, IxDyn};

use crate::TensorComputationError;

#[async_trait]
pub trait TensorComputation {
    type Input;
    type Output;

    async fn add(&self, other: &Self) -> crate::Result<Self::Output, TensorComputationError>;

    async fn multiply(&self, other: &Self) -> crate::Result<Self::Output, TensorComputationError>;

    async fn transpose(&self) -> crate::Result<Self::Output, TensorComputationError>;
}

pub struct Tensor<T> {
    data: ArrayD<T>,
}

impl<T> Tensor<T>
where T: Clone
{
    pub fn new<D, I>(shape: D, values: I) -> crate::Result<Self, TensorComputationError>
    where
        D: Into<IxDyn>,
        I: IntoIterator<Item = T>,
    {
        let shape: IxDyn = shape.into();
        let values: Vec<T> = values.into_iter().collect();
        let expected_size = shape.size();

        if values.len() != expected_size {
            return Err(TensorComputationError::ShapeMismatch {
                expected: vec![expected_size],
                found: vec![values.len()],
            });
        }

        let data = ArrayD::from_shape_vec(shape, values)
            .map_err(|source| TensorComputationError::InvalidShape { source })?;

        Ok(Tensor { data })
    }

    pub fn shape(&self) -> &[usize] {
        self.data.shape()
    }

    pub fn get(&self, index: &[usize]) -> Option<&T> {
        self.data.get(index)
    }
}

#[async_trait]
impl<T> TensorComputation for Tensor<T>
where T: Clone + Send + Sync + 'static + std::ops::Add<Output = T> + std::ops::Mul<Output = T>
{
    type Input = Tensor<T>;
    type Output = Tensor<T>;

    async fn add(
        &self,
        other: &Self::Input,
    ) -> crate::Result<Self::Output, TensorComputationError> {
        if self.shape() != other.shape() {
            return Err(TensorComputationError::ShapeMismatch {
                expected: self.shape().to_vec(),
                found: other.shape().to_vec(),
            });
        }

        let result_data = self
            .data
            .iter()
            .zip(&other.data)
            .map(|(a, b)| a.clone() + b.clone())
            .collect::<Vec<_>>();

        Ok(Tensor::new(IxDyn(self.shape()), result_data)?)
    }

    async fn multiply(
        &self,
        other: &Self::Input,
    ) -> crate::Result<Self::Output, TensorComputationError> {
        if self.shape() != other.shape() {
            return Err(TensorComputationError::ShapeMismatch {
                expected: self.shape().to_vec(),
                found: other.shape().to_vec(),
            });
        }

        let result_data = self
            .data
            .iter()
            .zip(&other.data)
            .map(|(a, b)| a.clone() * b.clone())
            .collect::<Vec<_>>();

        Ok(Tensor::new(IxDyn(self.shape()), result_data)?)
    }

    async fn transpose(&self) -> crate::Result<Self::Output, TensorComputationError> {
        let transposed_data = self.data.t().to_owned();
        Ok(Tensor {
            data: transposed_data,
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[async_std::test]
    #[cfg_attr(miri, ignore)]
    async fn test_tensor_creation() {
        let tensor = Tensor::new(IxDyn(&[2, 2]), [1.0, 2.0, 3.0, 4.0]).unwrap();
        assert_eq!(tensor.shape(), &[2, 2]);
        assert_eq!(tensor.get(&[0, 0]), Some(&1.0));
        assert_eq!(tensor.get(&[1, 1]), Some(&4.0));
    }

    #[async_std::test]
    #[cfg_attr(miri, ignore)]
    async fn test_tensor_addition() {
        let tensor1 = Tensor::new(IxDyn(&[2, 2]), [1.0, 2.0, 3.0, 4.0]).unwrap();
        let tensor2 = Tensor::new(IxDyn(&[2, 2]), [5.0, 6.0, 7.0, 8.0]).unwrap();

        let result = tensor1.add(&tensor2).await.unwrap();
        assert_eq!(result.shape(), &[2, 2]);
        assert_eq!(result.get(&[0, 0]), Some(&6.0));
        assert_eq!(result.get(&[1, 1]), Some(&12.0));
    }

    #[async_std::test]
    #[cfg_attr(miri, ignore)]
    async fn test_tensor_multiplication() {
        let tensor1 = Tensor::new(IxDyn(&[2, 2]), [1.0, 2.0, 3.0, 4.0]).unwrap();
        let tensor2 = Tensor::new(IxDyn(&[2, 2]), [5.0, 6.0, 7.0, 8.0]).unwrap();

        let result = tensor1.multiply(&tensor2).await.unwrap();
        assert_eq!(result.shape(), &[2, 2]);
        assert_eq!(result.get(&[0, 0]), Some(&5.0));
        assert_eq!(result.get(&[1, 1]), Some(&32.0));
    }

    #[async_std::test]
    #[cfg_attr(miri, ignore)]
    async fn test_tensor_transpose() {
        let tensor = Tensor::new(IxDyn(&[2, 3]), [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();

        let result = tensor.transpose().await.unwrap();
        assert_eq!(result.shape(), &[3, 2]);
        assert_eq!(result.get(&[0, 0]), Some(&1.0));
        assert_eq!(result.get(&[1, 0]), Some(&2.0));
        assert_eq!(result.get(&[0, 1]), Some(&4.0));
    }

    #[async_std::test]
    #[cfg_attr(miri, ignore)]
    async fn test_shape_mismatch_error() {
        let tensor1 = Tensor::new(IxDyn(&[2, 2]), [1.0, 2.0, 3.0, 4.0]).unwrap();
        let tensor2 = Tensor::new(IxDyn(&[3, 2]), [5.0, 6.0, 7.0, 8.0, 9.0, 10.0]).unwrap();

        let result = tensor1.add(&tensor2).await;
        assert!(result.is_err());

        if let Err(TensorComputationError::ShapeMismatch { expected, found }) = result {
            assert_eq!(expected, vec![2, 2]);
            assert_eq!(found, vec![3, 2]);
        } else {
            panic!("Expected a ShapeMismatch error");
        }
    }

    #[async_std::test]
    #[cfg_attr(miri, ignore)]
    async fn test_invalid_shape_error() {
        let result = Tensor::new(IxDyn(&[2, 2]), [1.0, 2.0, 3.0]); // 数据不足
        assert!(result.is_err());

        if let Err(TensorComputationError::ShapeMismatch { expected, found }) = result {
            assert_eq!(expected, vec![4]);
            assert_eq!(found, vec![3]);
        } else {
            panic!("Expected a ShapeMismatch error");
        }
    }
}
