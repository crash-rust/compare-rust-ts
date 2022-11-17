export function results() {
  //! 在ts当中一般使用try。。catch去定义错误异常，或者在业务当中定义业务码区分业务异常
  let response = login();
  if (response.code == 401)
    console.log(`response message is ${response.message}`);

  try {
    readFile(true);
  } catch (err) {
    console.log(`err is ${err}`);
  }

  let boundaryNum = readArrBoundary([1, 2, 3]);

  let numRes = boundaryNum + 1;

  // !虽然错了，但是在ts当中这里可以运行起来，工程上很容易导致一些bug出现
  console.log(`numsRes is ${numRes}`);
}

function login() {
  return {
    code: 401,
    message: 'no login'
  };
}

function readFile(pathExist: boolean) {
  if (pathExist) {
    return '~/.cargo/config';
  } else {
    throw new Error('can not read property of undeinfed: read path');
  }
}

function readArrBoundary(nums: number[]): number {
  return nums[nums.length];
}
