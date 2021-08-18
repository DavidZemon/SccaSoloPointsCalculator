import { EOL } from 'os';
import { Component, ComponentPropsWithoutRef } from 'react';
import { Accordion, Button, Card, Col, Row, Table } from 'react-bootstrap';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faDownload } from '@fortawesome/free-solid-svg-icons';
import {
  ClassResults,
  Driver,
  EventResults as EventResultsData,
  ShortCarClass,
} from '../models';
import { calculatePointsForDriver } from '../services';
import { RamDownload } from './DownloadButton';

interface EventResultsProps extends ComponentPropsWithoutRef<any> {
  results?: EventResultsData;
}

interface EventResultsState {
  csvContent?: string;
  exportFilename?: string;
}

export class EventResults extends Component<
  EventResultsProps,
  EventResultsState
> {
  private static readonly MAX_LAP_COUNT = 10;

  constructor(props: Readonly<EventResultsProps>) {
    super(props);
    this.state = {};
  }

  public render() {
    if (this.props.results) {
      return [
        <Row key={0} className={'top-buffer'}>
          <Col>
            <h2>Event Results</h2>

            <Accordion>
              <Card>
                <Card.Header key={'class'}>
                  <Accordion.Toggle
                    eventKey={'class'}
                    as={Button}
                    variant={'link'}
                  >
                    Results by Class
                  </Accordion.Toggle>
                  <Button variant={'secondary'}>
                    <FontAwesomeIcon
                      className={'clickable'}
                      icon={faDownload}
                      onClick={() => this.exportFullResultsToCsv()}
                    />
                  </Button>
                </Card.Header>

                <Accordion.Collapse eventKey={'class'}>
                  <Card.Body>
                    <Accordion>
                      {Object.entries(this.props.results).map(
                        ([carClass, classResults], index) => (
                          <Card key={index}>
                            <Card.Header>
                              <Accordion.Toggle
                                eventKey={`${index}`}
                                as={Button}
                                variant={'link'}
                              >
                                {classResults.carClass.long}
                              </Accordion.Toggle>
                            </Card.Header>

                            <Accordion.Collapse eventKey={`${index}`}>
                              <Card.Body>
                                <Table key={index} striped hover borderless>
                                  <thead>
                                    <tr>
                                      <th>RK</th>
                                      <th>Pos</th>
                                      <th>Nbr</th>
                                      <th>Name</th>
                                      <th>Car</th>
                                      <th>Region</th>
                                      <th colSpan={EventResults.MAX_LAP_COUNT}>
                                        Lap Times
                                      </th>
                                      <th>Best Lap</th>
                                      <th>Difference</th>
                                    </tr>
                                  </thead>
                                  <tbody>
                                    {this.displayClassResultsRows(classResults)}
                                  </tbody>
                                </Table>
                              </Card.Body>
                            </Accordion.Collapse>
                          </Card>
                        ),
                      )}
                    </Accordion>
                  </Card.Body>
                </Accordion.Collapse>
              </Card>

              {this.displayCombinedResults('PAX')}

              {this.displayCombinedResults('Raw')}

              {this.displayCombinedResults('Novice')}
            </Accordion>
          </Col>
        </Row>,
        <RamDownload
          key={1}
          filename={this.state.exportFilename}
          content={this.state.csvContent}
          contentType={'text/csv'}
          downloadComplete={() =>
            this.setState({ csvContent: undefined, exportFilename: undefined })
          }
        />,
      ];
    } else {
      return null;
    }
  }

  private displayClassResultsRows(classResults: ClassResults): JSX.Element[] {
    const bestTimeOfDay = classResults.getBestInClass();
    return classResults.drivers.map((driver, index) => {
      return (
        <tr key={index}>
          <td>{driver.rookie ? 'R' : ''}</td>
          <td>{driver.position}</td>
          <td>{driver.carNumber}</td>
          <td>{driver.name}</td>
          <td>{driver.carDescription}</td>
          <td>{driver.region}</td>
          {driver.getTimes()!.map((lapTime, index) => (
            <td key={`day1Time-${index}`}>{lapTime.toString()}</td>
          ))}
          {new Array(EventResults.MAX_LAP_COUNT - driver.getTimes()!.length)
            .fill(null)
            .map((_, index) => (
              <td key={`day1TimeFiller-${index}`} />
            ))}
          <td>{driver.bestLap().toString()}</td>
          <td>{driver.difference(bestTimeOfDay)}</td>
        </tr>
      );
    });
  }

  private displayCombinedResults(
    resultsType: 'PAX' | 'Raw' | 'Novice',
  ): JSX.Element {
    const drivers = Object.values(this.props.results!)
      .map((classResults) => classResults.drivers)
      .flat();
    const sortedDrivers = drivers
      .map(
        (driver) =>
          [
            driver,
            (driver.bestLap().time || Infinity) *
              (resultsType === 'Raw' ? 1 : driver.paxMultiplier),
          ] as [Driver, number],
      )
      .sort(([_1, d1Time], [_2, d2Time]) => d1Time - d2Time)
      .filter(([driver, _]) => {
        switch (resultsType) {
          case 'Novice':
            return driver.rookie;
          case 'PAX':
            return 'FUN' !== driver.carClass.short;
          case 'Raw':
            return true;
        }
        throw new Error(`Unrecognized eventResultsType: ${resultsType}`);
      });
    const fastestOfDay = sortedDrivers[0][1];
    return (
      <Card>
        <Card.Header key={resultsType}>
          <Accordion.Toggle eventKey={resultsType} as={Button} variant={'link'}>
            {resultsType} Results
          </Accordion.Toggle>
          <Button
            variant={'secondary'}
            onClick={() =>
              this.exportCombinedResultsToCsv(
                sortedDrivers,
                fastestOfDay,
                resultsType,
              )
            }
          >
            <FontAwesomeIcon className={'clickable'} icon={faDownload} />
          </Button>
        </Card.Header>

        <Accordion.Collapse eventKey={resultsType}>
          <Card.Body>
            <Table striped hover borderless>
              <thead>
                <tr>
                  <th>Position</th>
                  <th>Class</th>
                  <th>Car #</th>
                  <th>Name</th>
                  <th>Car</th>
                  <th>Region</th>
                  <th>{resultsType ? 'PAX' : 'Raw Corrected'} Time</th>
                  <th>Difference</th>
                </tr>
              </thead>

              <tbody>
                {sortedDrivers.map(([driver, driverBestTime], index) => (
                  <tr key={index}>
                    <td>{index + 1}</td>
                    <td>{driver.carClass.short}</td>
                    <td>{driver.carNumber}</td>
                    <td>{driver.name}</td>
                    <td>{driver.carDescription}</td>
                    <td>{driver.region}</td>
                    <td>
                      {driver
                        .bestLap()
                        .toString(
                          resultsType === 'Raw'
                            ? undefined
                            : driver.paxMultiplier,
                        )}
                    </td>
                    <td>
                      {driver.difference(fastestOfDay, resultsType !== 'Raw')}
                    </td>
                  </tr>
                ))}
              </tbody>
            </Table>
          </Card.Body>
        </Accordion.Collapse>
      </Card>
    );
  }

  private exportResultsByClassCsv() {
    const lines = [
      [
        'Pos',
        'Name',
        'Car',
        'Class',
        'Number',
        'Total Time',
        'Index Time',
        'From Previous',
        'From Top',
        'Points',
        'Region',
      ],
      ...Object.values(this.props.results!)
        .map((classResults) => this.exportClassResultsToCsv(classResults))
        .flat(),
    ];
    this.setState({
      exportFilename: 'event_results.csv',
      csvContent: lines.map((line) => `"${line.join('","')}"`).join(EOL),
    });
  }

  private exportClassResultsToCsv(classResults: ClassResults): string[][] {
    const shortCarClass = classResults.carClass.short;
    const bestLapInClass = classResults.getBestInClass();
    const bestIndexTime =
      (bestLapInClass || Infinity) * classResults.drivers[0].paxMultiplier;
    return [
      [`${shortCarClass} - ${classResults.carClass}`],
      ...classResults.drivers.map((driver, index) => {
        return [
          `${driver.position}`,
          driver.name,
          driver.carDescription,
          shortCarClass,
          `${driver.carNumber}`,
          driver.bestLap().toString(undefined, false),
          driver.bestLap().toString(driver.paxMultiplier, false),
          index === 0
            ? ''
            : driver.difference(
                (classResults.drivers[index - 1].bestLap().time || Infinity) *
                  driver.paxMultiplier,
                true,
              ),
          driver.difference(bestIndexTime, true),
          `${calculatePointsForDriver(
            bestIndexTime,
            driver,
            driver.paxMultiplier,
          )}`,
          driver.region,
        ];
      }),
    ];
  }

  private exportCombinedResultsToCsv(
    sortedDrivers: [Driver, number][],
    fastestOfDay: number,
    resultsType: 'PAX' | 'Raw' | 'Novice',
  ): void {
    const isRawTime = resultsType === 'Raw';
    const results = [
      [
        'Position',
        'Name',
        'Car',
        'Class',
        'Car #',
        `${isRawTime ? 'Best' : 'Index'} Time`,
        'From Previous',
        'From Top',
        ...(isRawTime ? [] : ['Points']),
      ],
      ...sortedDrivers.map(([driver, time], index) => {
        const previousDriver = sortedDrivers[index - 1][0];
        return [
          `${index + 1}`,
          driver.name,
          driver.carDescription,
          driver.carClass.short,
          `${driver.carNumber}`,
          driver
            .bestLap('combined')
            .toString(isRawTime ? undefined : driver.paxMultiplier, false),
          index === 0
            ? ''
            : driver.difference(
                (previousDriver.bestLap().time || Infinity) *
                  (isRawTime ? 1 : previousDriver.paxMultiplier),
                !isRawTime,
              ),
          driver.difference(fastestOfDay, !isRawTime),
          ...(isRawTime
            ? []
            : [
                calculatePointsForDriver(
                  fastestOfDay,
                  driver,
                  driver.paxMultiplier,
                ),
              ]),
        ];
      }),
    ];
    this.setState({
      exportFilename: `event_${resultsType.toLowerCase()}_results.csv`,
      csvContent: results.map((row) => `"${row.join('","')}"`).join(EOL),
    });
  }

  private exportFullResultsToCsv(): void {
    const lines = (
      Object.entries(this.props.results!) as [ShortCarClass, ClassResults][]
    )
      .map(([carClass, classResults]) => {
        const bestInClass = classResults.drivers[0].bestLap().time;
        return [
          [`${carClass} (Trophies: ${classResults.trophyCount})`],
          [
            'RK',
            'Pos',
            'Nbr',
            "Driver's name, Town",
            'Car, Sponsor',
            'Region',
            'Times',
            ...new Array(10).fill(null).map(() => ''),
            'Best Lap',
            'Difference',
            'From Previous',
          ],
          ...classResults.drivers.map((driver) => {
            const previousDriver = classResults.drivers[driver.position! - 2];
            return [
              driver.rookie ? 'R' : '',
              `${driver.position}`,
              driver.carNumber,
              driver.name,
              driver.carDescription,
              driver.region,
              ...driver.getTimes('day1')!,
              '',
              driver.bestLap(),
              driver.position === 1 ? '' : driver.difference(bestInClass),
              driver.position === 1
                ? ''
                : driver.difference(previousDriver.bestLap().time),
            ];
          }),
        ];
      })
      .flat();
    this.setState({
      exportFilename: 'event_results.csv',
      csvContent: lines.map((line) => `"${line.join('","')}"`).join(EOL),
    });
  }
}
